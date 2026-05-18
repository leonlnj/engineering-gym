import os
from pyflink.table import EnvironmentSettings, TableEnvironment

def run_kafka_job():
    # 1. Setup Table Environment
    env_settings = EnvironmentSettings.in_streaming_mode()
    t_env = TableEnvironment.create(env_settings)

    # 2. Define Source (Kafka)
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

    print("Submitting SQL transformation...")
    t_env.execute_sql("""
        INSERT INTO kafka_sink
        SELECT user_id, UPPER(payload)
        FROM kafka_source
    """).wait()

if __name__ == '__main__':
    run_kafka_job()