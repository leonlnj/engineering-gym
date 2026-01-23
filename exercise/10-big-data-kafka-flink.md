# Exercise 10 - Big Data Processing with Kafka and Flink

This exercise demonstrates a real-time ETL pipeline. JSON data is ingested from a Kafka topic, transform it using a PyFlink job, and sink the results back into a different Kafka topic.

## Flink Concept

Apache Flink is a distributed processing engine designed for stateful computations over data streams. In a Kubernetes-native environment, the architecture is divided into a control plane and a data plane.

### Components
JobManager (The Orchestrator): Acts as the master node. It coordinates the data flow graph, manages checkpoints, and handles recovery in the event of a failure.

TaskManager (The Worker): Executes the actual data processing tasks. Each TaskManager provides "Task Slots," which represent the fixed resource capacity (CPU/Memory) available to execute parallel slices of the data.

### Unified Stream Processing via Table API
The exercise utilizes the Table API, a high-level relational abstraction. This allows the developer to treat a continuous Kafka stream as a "Dynamic Table."

Concept: Instead of writing complex procedural code, SQL-like logic is applied to the stream.

Application: The transformation UPPER(payload) is treated as a continuous query that produces a new stream of data as soon as records arrive at the source.

### Deployment Mode
The deployment follows the Application Mode pattern facilitated by the Flink Kubernetes Operator.

Concept: Unlike Session Mode (where a cluster exists to run multiple jobs), Application Mode creates a 1-to-1 relationship between the Flink cluster and the specific Python job.

Application: This ensures resource isolation and simplifies dependency management, as the Python environment and Kafka connectors are bundled directly into the container image.

### Connector Ecosystem and Deserialization
Flink is inherently storage-agnostic and requires external libraries to interface with message brokers.

Concept: The Kafka SQL Connector acts as the bridge. It handles the low-level polling of Kafka partitions and the "Serialization/Deserialization" (SerDe) process.

Application: In this exercise, the connector converts raw bytes from Kafka into structured Flink Rows based on the JSON schema defined in the CREATE TABLE DDL.

### Stateless ETL (Extract, Transform, Load)
The pipeline performs a stateless transformation.

Concept: A transformation is stateless if the processing of one event does not depend on any information from previous events.

Application: Converting a string to uppercase is a classic stateless operation. Flink processes each record independently, which allows for high throughput and simple scaling.

## Setup

Required files are created under `./resources/ex-10`. Working dir will assumed to use that. The kafka namespace will be used for this exercise.

- Kafka
  - Install Strimzi Operator
  - Create a Basic cluster
- Flink
  - Create a Flink Python job
  - Create a container image with python job and kafka connector
  - Build and tag the image as `flink-kafka:2.0.1`, load image into minikube
  - Install Flink Operator via Helm
  - Deploy a `FlinkDeployment` CRD

### Kafka Setup

[Strimzi](https://strimzi.io/) operator will be used to manage Kafka.

```sh
# https://strimzi.io/quickstarts/
kubectl create namespace kafka
kubectl create -f 'https://strimzi.io/install/latest?namespace=kafka' -n kafka
kubectl get pod -n kafka --watch
# Create a single node kafka
kubectl apply -f https://strimzi.io/examples/latest/kafka/kafka-single-node.yaml -n kafka 
# Create topics required for the exercise
kubectl apply -f topics.yaml
```

### Flink setup

#### Flink Operator

Install the [operator](https://nightlies.apache.org/flink/flink-kubernetes-operator-docs-main/docs/operations/helm/) via Helm to manage the lifecycle of Flink jobs.

```sh
helm repo add flink-operator-repo https://downloads.apache.org/flink/flink-kubernetes-operator-1.13.0/
helm install flink-kubernetes-operator flink-operator-repo/flink-kubernetes-operator \
--set webhook.create=false \
-n kafka 
```

#### Flink Python Job

This script defines the source table (input topic), the sink table (output topic), and the transformation logic.

It reads a json with `user_id` and `payload` key, does a transformation to uppercase the payload.

```python
import os
from pyflink.table import EnvironmentSettings, TableEnvironment

def run_kafka_job():
    env_settings = EnvironmentSettings.in_streaming_mode()
    t_env = TableEnvironment.create(env_settings)

    # Define Source (Kafka)
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
            'format' = 'json',
            'json.ignore-parse-errors' = 'true',
            'json.fail-on-missing-field' = 'false'
        )
    """)

    # Define Sink (Kafka)
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

    print("Submitting SQL transformation...")
    t_env.execute_sql("""
        INSERT INTO kafka_sink
        SELECT user_id, UPPER(payload)
        FROM kafka_source
    """).wait()

if __name__ == '__main__':
    run_kafka_job()
```

#### Dockerfile 

> Flink does not come with connector and must be built manually ([ref](https://nightlies.apache.org/flink/flink-docs-release-1.17/docs/deployment/resource-providers/standalone/docker))


This dockerfile installs python, apache-flink and flink-kafka-connector.

```docker
FROM flink:2.0.1-scala_2.12-java21

# Use root to install system packages
USER root

# Combine system updates, wget, and clean-up to keep the layer small
RUN apt-get update -y && \
    apt-get install -y --no-install-recommends python3 python3-pip wget && \
    ln -s /usr/bin/python3 /usr/bin/python && \
    # Download Kafka Connector directly into flink lib
    wget -P /opt/flink/lib/ https://repo1.maven.org/maven2/org/apache/flink/flink-sql-connector-kafka/4.0.1-2.0/flink-sql-connector-kafka-4.0.1-2.0.jar && \
    rm -rf /var/lib/apt/lists/*

# 2. Python Dependencies
RUN pip3 install --no-cache-dir apache-flink

# 3. App code and Permissions
WORKDIR /opt/flink/usrlib
COPY --chown=flink:flink job.py .

# 4. Environment Variables (Required for PyFlink to locate Python)
ENV PYFLINK_PYTHON=/usr/bin/python3
ENV PYFLINK_CLIENT_PYTHON=/usr/bin/python3

USER flink
```

Build and load the image:

```sh
docker build -t flink-kafka:2.0.1 .
minikube image load flink-kafka:2.0.1
```

#### FlinkDeployment

Deploy the FlinkDeployment Custom Resource. This defines the resources and specifies the entry point for the Python script.

```sh
kubectl apply -f flink.yaml
```

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
      memory: "800Mi"
      cpu: 0.3
  job:
    jarURI: local:///opt/flink/opt/flink-python-2.0.1.jar
    entryClass: "org.apache.flink.client.python.PythonDriver"
    args: ["-py", "/opt/flink/usrlib/job.py"]
    parallelism: 1
    upgradeMode: stateless
    state: running
```

## Test

To verify that the Flink job is correctly transforming data, use two separate terminals to act as a Producer (sending data) and a Consumer (receiving results).

```sh
# run a consumer to watch `output-topic`
kubectl -n kafka run kafka-consumer -ti --image=quay.io/strimzi/kafka:0.50.0-kafka-4.0.1 --rm=true --restart=Never -- bin/kafka-console-consumer.sh --bootstrap-server my-cluster-kafka-bootstrap:9092 --topic output-topic
# in a separate terminal, run a producer that writes to `input-topic`
kubectl -n kafka run kafka-producer -ti --image=quay.io/strimzi/kafka:0.50.0-kafka-4.0.1 --rm=true --restart=Never -- bin/kafka-console-producer.sh --bootstrap-server my-cluster-kafka-bootstrap:9092 --topic input-topic
```

In the producer console, send the following Json:

```json
{"user_id": "user_123", "payload": "hello flink"}
```

In the consumer console, validate the transformed output:

```json
{"user_id":"user_123","result_payload":"HELLO FLINK"}
```

## Cleanup

```sh
kubectl delete flinkdeployment flink-cluster
helm uninstall flink-kubernetes-operator -n kafka
kubectl delete rolebinding flink-role-binding -n kafka
kubectl delete role flink -n kafka
kubectl delete serviceaccount flink -n kafka
minikube image rm flink-kafka:2.0.1
kubectl delete -f https://strimzi.io/examples/latest/kafka/kafka-single-node.yaml -n kafka 
kubectl delete -f 'https://strimzi.io/install/latest?namespace=kafka' -n kafka
kubectl delete pvc data-0-my-cluster-dual-role-0 -n kafka
kubectl delete namespace kafka
```
