# Exercise 2 - Cache

Setup cache and do some measurement of performance gain

## Setup

- Install a bitnami redis helm chart
- Run a python pod with inline python script against redis


```sh
# Install Redis without auth
helm install ex-2 oci://registry-1.docker.io/bitnamicharts/redis \
  --set architecture=standalone \
  --set auth.enabled=false
```

## Test

Run the shell command below. The inline Python script performs three steps:

1. Run the slow function (sleep and return square value) 50 times without caching – simulates the baseline computation.
2. Run the function 50 times with Redis caching (cold cache) – for each item, it tries to get the value from Redis; if the key is missing, it computes the value and stores it in Redis with a 1-minute expiry.
3. Run the function 50 times with Redis caching (warm cache) – all values are already in Redis, so the function is not executed; only Redis GET operations are performed.

```sh
kubectl run redis-demo \
  --rm -i --restart=Never \
  --image=python:3.12-slim \
  --command -- sh -c "
    pip install redis >/dev/null && \
    python3 -c '
import time, redis, uuid

# Init Redis Client
r=redis.Redis(host=\"ex-2-redis-master\",port=6379,decode_responses=True)

# Unique ID for this run to avoid reusing old cache keys
run_id = uuid.uuid4().hex[:8]

# Test function: simulates an expensive IO/computation by sleeping then returns the square of the input
f=lambda x:(time.sleep(0.05),x*x)[1]

# Redis-backed version of the test function
# For each input x:
# 1. Try to get the value from Redis
# 2. If missing, compute with f(x) and store it in Redis with 60s expiry
f_cache = lambda x: (
    r.get(k := f\"sq:{run_id}:{x}\")
    or r.set(k, f(x), ex=60)
)

n=50 # Number of iterations

# Step 1: Run the function without caching
t=time.time()
[f(i) for i in range(n)]
t1=time.time()-t

# Step 2: Run with Redis caching (cold cache) – first pass populates Redis
t=time.time()
[f_cache(i) for i in range(n)]
t2=time.time()-t

# Step 3: Run with Redis caching (warm cache) – values already in Redis
t=time.time()
[f_cache(i) for i in range(n)]
t3=time.time()-t

print(f\"No cache: {t1:.2f}s\\nRedis cold: {t2:.2f}s\\nRedis warm: {t3:.2f}s\")
'"
```

Sample result

```text
No cache: 2.54s
Redis cold: 2.62s
Redis warm: 0.01s
```

## Cleanup

```
helm uninstall ex-2 
kubectl get pvc | grep 'redis' | awk '{print $1}' | xargs kubectl delete pvc 
```

## Appendix

### How to portforward to redis for local/out of cluster access

```sh
kubectl port-forward --namespace default svc/ex-2-redis-master 6379:6379 
```

### Running interactive redis client

Use the server directly

```sh
kubectl exec -it $(kubectl get pods -l app.kubernetes.io/name=redis -o jsonpath='{.items[0].metadata.name}') -- redis-cli
# in the Redis pod terminal
SET test "hello"
GET test
exit
```