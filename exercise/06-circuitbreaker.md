# Exercise 6 - Circuitbreaker

Simple exercise running inline python on a pod

## Concept

A circuit breaker tracks recent failures and moves between three states:

- **Closed**: normal operation; failures are counted; circuit opens after `fail_max` consecutive failures.  
- **Open**: calls are blocked immediately; no work is attempted; transitions to Half-Open after `reset_timeout`.  
- **Half-Open**: allows one trial call; success → closes circuit, failure → reopens immediately.  

> Note: Half-Open is stricter — a single failure reopens the circuit. Not all failures may appear in logs.

## Test

Execute the shell command below to run a python container with inline python script to demo circuitbreaker with `pybreaker` library.

```python
kubectl run pybreaker-demo \
  --image=python:3.12-slim \
  --rm -i --restart=Never \
  --command -- sh -c '
pip install pybreaker && python3 -c "
import pybreaker, random, time

# Listener attached to circuitbreaker to print state change
class Listener(pybreaker.CircuitBreakerListener):
    def state_change(self, cb, old, new):
        print(f\"⚡ {old.__class__.__name__} → {new.__class__.__name__}\")

# Circuitbreaker object with max failure, timeout reset counter and listener attached
breaker = pybreaker.CircuitBreaker(
    fail_max=2,
    reset_timeout=3,
    listeners=[Listener()]
)

@breaker
def flaky():
    if random.random() < 0.6:
        raise Exception(\"err\")
    return \"ok\"

for i in range(1, 30):
    try:
        print(f\"Call {i}:\", flaky())
    except pybreaker.CircuitBreakerError:
        print(f\"Call {i}: ⛔ OPEN\")
    except Exception:
        print(f\"Call {i}: ❌ fail\")
    time.sleep(1)
"'
```

Sample output

```sh
Call 1: ❌ fail
⚡ CircuitClosedState → CircuitOpenState
Call 2: ⛔ OPEN
Call 3: ⛔ OPEN
Call 4: ⛔ OPEN
⚡ CircuitOpenState → CircuitHalfOpenState
⚡ CircuitHalfOpenState → CircuitClosedState
Call 5: ok
Call 6: ok
Call 7: ❌ fail
Call 8: ok
Call 9: ok
Call 10: ❌ fail
⚡ CircuitClosedState → CircuitOpenState
Call 11: ⛔ OPEN
Call 12: ⛔ OPEN
Call 13: ⛔ OPEN
⚡ CircuitOpenState → CircuitHalfOpenState
⚡ CircuitHalfOpenState → CircuitClosedState
Call 14: ok
Call 15: ok
Call 16: ❌ fail
⚡ CircuitClosedState → CircuitOpenState
Call 17: ⛔ OPEN
Call 18: ⛔ OPEN
Call 19: ⛔ OPEN
⚡ CircuitOpenState → CircuitHalfOpenState
⚡ CircuitHalfOpenState → CircuitOpenState
Call 20: ⛔ OPEN
Call 21: ⛔ OPEN
Call 22: ⛔ OPEN
⚡ CircuitOpenState → CircuitHalfOpenState
⚡ CircuitHalfOpenState → CircuitOpenState
Call 23: ⛔ OPEN
Call 24: ⛔ OPEN
Call 25: ⛔ OPEN
⚡ CircuitOpenState → CircuitHalfOpenState
⚡ CircuitHalfOpenState → CircuitClosedState
Call 26: ok
Call 27: ok
Call 28: ❌ fail
⚡ CircuitClosedState → CircuitOpenState
Call 29: ⛔ OPEN
```