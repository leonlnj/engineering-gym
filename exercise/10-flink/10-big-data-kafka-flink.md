# Exercise 10 - Big Data Processing with Kafka and Flink

Setup Kafka and do some basic operations.

## Setup

### Kafka Setup

[Strimzi](https://strimzi.io/) operator will be used. Kafka will be deployed via Strimzi CRD.

```sh
# https://strimzi.io/quickstarts/
kubectl create namespace kafka
kubectl create -f 'https://strimzi.io/install/latest?namespace=kafka' -n kafka
# Run the below an d wait for pod to be ready
kubectl get pod -n kafka --watch
kubectl apply -f https://strimzi.io/examples/latest/kafka/kafka-single-node.yaml -n kafka 
```

### Flink setup

Flink does not come with connector and must be built manually ([ref](https://nightlies.apache.org/flink/flink-docs-release-1.17/docs/deployment/resource-providers/standalone/docker))

Steps
- Create a dockerfile
- Build and tag the image as `flink-kafka:1.17.2`, load image into minikube
- Install Flink Operator via Helm
- Create a configmap for python job
- Deploy a `FlinkDeployment`

#### Dockerfile 

Build an image with python 

```docker
FROM flink:2.0.1-scala_2.12-java21

USER root

ENV JAVA_HOME=/opt/java/openjdk
ENV PATH="${JAVA_HOME}/bin:${PATH}"

RUN apt-get update -y && \
    apt-get install -y --no-install-recommends python3 python3-pip wget && \
    ln -s /usr/bin/python3 /usr/bin/python && \
    rm -rf /var/lib/apt/lists/*

RUN pip3 install --no-cache-dir py4j==0.10.9.7

RUN wget -P /opt/flink/lib/ https://repo1.maven.org/maven2/org/apache/flink/flink-sql-connector-kafka/4.0.1-2.0/flink-sql-connector-kafka-4.0.1-2.0.jar

# Use the PyFlink library already inside the image
ENV PYTHONPATH=/opt/flink/opt/python/pyflink.zip

USER flink
```
#### Build and load image

```sh
docker build -t flink-kafka:2.0.1 .
minikube image load flink-kafka:2.0.1
```

#### Install Operator via Helm

Install the Flink operator via helm 
- https://nightlies.apache.org/flink/flink-kubernetes-operator-docs-main/docs/operations/helm/
- https://nightlies.apache.org/flink/flink-kubernetes-operator-docs-release-1.13/docs/try-flink-kubernetes-operator/quick-start/

```sh
helm repo add flink-operator-repo https://downloads.apache.org/flink/flink-kubernetes-operator-1.13.0/
helm install \
--set webhook.create=false \
flink-kubernetes-operator flink-operator-repo/flink-kubernetes-operator -n kafka 
```

#### Create a python script as configmap

```python
apiVersion: v1
kind: ConfigMap
metadata:
  name: flink-job
data:
  job.py: |
    import os
    import sys
    
    # Ensure the internal zip is in the path for the runtime
    sys.path.insert(0, '/opt/flink/opt/python/pyflink.zip')
    
    from pyflink.table import EnvironmentSettings, TableEnvironment

    def run_kafka_job():
        # 1. Setup Table Environment
        env_settings = EnvironmentSettings.in_streaming_mode()
        t_env = TableEnvironment.create(env_settings)

        # 2. Define Source (Kafka)
        # Note: 'kafka-service' should be your K8s service name for Kafka
        t_env.execute_sql("""
            CREATE TABLE kafka_source (
                user_id STRING,
                payload STRING
            ) WITH (
                'connector' = 'kafka',
                'topic' = 'input-topic',
                'properties.bootstrap.servers' = 'my-cluster-kafka-bootstrap:9092',
                'properties.group.id' = 'flink-group',
                'scan.startup.mode' = 'earliest-offset',
                'format' = 'json'
            )
        """)

        # 3. Define Sink (Kafka)
        t_env.execute_sql("""
            CREATE TABLE kafka_sink (
                user_id STRING,
                result_payload STRING
            ) WITH (
                'connector' = 'kafka',
                'topic' = 'output-topic',
                'properties.bootstrap.servers' = 'my-cluster-kafka-bootstrap:9092',
                'format' = 'json'
            )
        """)

        # 4. Define the Job logic
        print("Submitting SQL transformation...")
        t_env.execute_sql("""
            INSERT INTO kafka_sink
            SELECT user_id, UPPER(payload)
            FROM kafka_source
        """)

    if __name__ == '__main__':
        run_kafka_job()
```

```sh
kubectl create configmap flink-job --from-file=flink_job.py=./flink_job.py
```

#### Deploy a FlinkDeployment

```yaml
apiVersion: flink.apache.org/v1beta1
kind: FlinkDeployment
metadata:
  name: flink-cluster
spec:
  image: flink-kafka:2.0.1
  flinkVersion: v2_0
  serviceAccount: flink
  flinkConfiguration:
    taskmanager.numberOfTaskSlots: "1"
    taskmanager.memory.managed.fraction: "0.1"
    taskmanager.network.memory.min: "16mb"
    taskmanager.network.memory.max: "16mb"
  jobManager:
    resource:
      memory: "800Mi"
      cpu: 0.3
  taskManager:
    resource:
      memory: "768Mi"
      cpu: 0.3
  podTemplate:
    spec:
      containers:
        - name: flink-main-container
          volumeMounts:
            - name: python-scripts-volume
              mountPath: /opt/flink/usrlib/job.py
              subPath: job.py
      volumes:
        - name: python-scripts-volume
          configMap:
            name: flink-job
  job:
    jarURI: /opt/flink/opt/flink-python-2.0.1.jar
    entryClass: "org.apache.flink.client.python.PythonDriver"
    args: ["-py", "/opt/flink/usrlib/job.py"]
    parallelism: 1
    upgradeMode: stateless
    state: running
```

## Test

Test will be done using two terminal to run a producer and consumer.

```sh
# run a producer
kubectl -n kafka run kafka-producer -ti --image=quay.io/strimzi/kafka:0.49.1-kafka-4.1.1 --rm=true --restart=Never -- bin/kafka-console-producer.sh --bootstrap-server my-cluster-kafka-bootstrap:9092 --topic my-topic
# in a separate terminal, run a consumer
kubectl -n kafka run kafka-consumer -ti --image=quay.io/strimzi/kafka:0.49.1-kafka-4.1.1 --rm=true --restart=Never -- bin/kafka-console-consumer.sh --bootstrap-server my-cluster-kafka-bootstrap:9092 --topic my-topic --from-beginning
```

Simply type messages in the producer prompt and see message arriving in the consumer terminal.

## Cleanup

K8

```sh

minikube image rm flink-kafka:1.17.2
kubectl delete -f https://strimzi.io/examples/latest/kafka/kafka-single-node.yaml -n kafka 
kubectl delete -f 'https://strimzi.io/install/latest?namespace=kafka' -n kafka
kubectl delete namespace kafka
```
