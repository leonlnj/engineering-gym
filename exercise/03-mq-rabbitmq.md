# Exercise 3 - Message Queue

Setup RabbitMQ and do some basic operations.

## Context

### RabbitMQ basics with Kafka comparison

#### Overview

| Kafka          | RabbitMQ            | Meaning                  |
| -------------- | ------------------- | ------------------------ |
| Producer       | Producer            | Sends messages           |
| Topic          | Exchange            | Entry point for messages |
| Partition      | Queue               | Where messages wait      |
| Consumer       | Consumer            | Receives messages        |
| Consumer group | Competing consumers | Load sharing             |
| Offset         | Acknowledgement     | Tracking progress        |

> Kafka topics ≠ RabbitMQ queues

Use RabbitMQ when:
- Low latency per message
- Complex routing rules
- Task queues / work distribution
- Request–reply patterns
- Strong delivery guarantees

Use Kafka when:
- Event streaming
- Replayability
- Long-term storage
- High throughput
- Stream processing

#### Exchange

RabbitMQ exchanges does the following and behave like a router (its not work like Kafka topic!)
- Receives messages from producers
- Decides which queue(s) get the message
- Does not store messages

Each exchange is binded to a queue and maybe include a routing key. All queue have a binding key to the exchange.  
It is possible to bind an exchange to another exchange (routing it)

There are 4 types of exchange
- direct
  - routing key exactly matches the binding key
- fanout
  - routing_key = ignored. Broadcast to all bounded key
- topic
  - routing_key uses regex to match binding key
- headers
  - uses header and regex pattern instead of routing key to match binding key


#### Queue

RabbitMQ queue
- Stores messages until consumed
- Messages are removed once acknowledged
- One message → one consumer (normally)

Kafka difference
- Kafka messages stay in the log
- Multiple consumers can read the same message

RabbitMQ is destructive reads while Kafka is immutable reads

#### Routing key

RabbitMQ routing key:
- Used by the exchange to decide routing
- String-based pattern matching

Kafka key
- Used to choose partition
- Guarantees ordering

#### Consuming and ack

RabbitMQ
- Consumer acks message
- If no ack → message is requeued
- Ensures at-least-once delivery

Kafka
- Consumer commits offset
- Message remains even after commit

RabbitMQ emphasizes on delivery gurantee while kafka ephasizes on replayability.

## Setup

- Install a bitnami RabbitMQ helm chart
- Portforward to RabbitMQ instance in k8
- Download and configure `rabbitmqadmin` to communicate with broker


```sh
helm install ex-3 oci://registry-1.docker.io/bitnamicharts/rabbitmq \
  --set auth.username=user \
  --set auth.password=password \
  --set persistence.enabled=false \
  --set replicaCount=1 \
  --set image.repository=bitnamilegacy/rabbitmq \
  --set global.security.allowInsecureImages=true \
  --wait
```

To view UI, portforward and visit localhost:15672.

```sh
kubectl port-forward svc/ex-3-rabbitmq-headless 15672:15672
```

Download the [rabbitmq python binary](https://github.com/rabbitmq/rabbitmqadmin-ng/releases) to local machine. This requires a python env.

```sh
chmod +x rabbitmqadmin
# Create a config file
cat <<EOF > admin.conf
[default]
username = "user"
password = "password"
EOF
# The default config points to localhost:15672, with portforwarding no further args are required for host etc
# Validate the cli tool
./rabbitmqadmin -c admin.config list queues
```

## Test

The test will be ran using `rabbitmqadmin` on the localhost (not within the rabbitmq pod). 


Set up exchange, queue and bind queue to exchange
```sh
./rabbitmqadmin -c admin.conf declare exchange --name demo-exchange --type direct
./rabbitmqadmin -c admin.conf declare queue --name demo-queue
./rabbitmqadmin -c admin.conf declare binding --source demo-exchange --destination demo-queue --destination-type queue --routing-key=demo
```

> You can view the created exchange and queue on the UI

Produce and Consume message from the exchange
```sh
# Produce message "hello rabbitmq"
./rabbitmqadmin -c admin.conf publish message --exchange demo-exchange --routing-key demo --payload "hello rabbitmq"
# Consume message without acknowledging, message will stay in queue
./rabbitmqadmin -c admin.conf get messages --queue demo-queue --ack-mode reject_requeue_true
# Consume message and acknowledge. Message will still be visible and deleted after read.
./rabbitmqadmin -c admin.conf get messages --queue demo-queue --ack-mode reject_requeue_false
# Consume message, no message retrieved.
./rabbitmqadmin -c admin.conf get messages --queue demo-queue --ack-mode reject_requeue_true
```

> You can produce and create message on the UI after clicking into the queue

## Cleanup

Local machine

```sh
rm rabbitmqadmin admin.conf
```

K8

```sh
helm uninstall ex-3
```
