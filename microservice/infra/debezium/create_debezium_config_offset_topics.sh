# see https://hub.docker.com/r/debezium/connect

# CONFIG_STORAGE_TOPIC 
#   This environment variable is required when running the Kafka Connect service. 
#   Set this to the name of the Kafka topic where the Kafka Connect services in the group store connector configurations. 
#   The topic must have a single partition and be highly replicated (e.g., 3x or more).
docker exec rustddd-kafka kafka-topics --create --if-not-exists --topic "debezium_connect_configs" --replication-factor 1 --partitions 1 --bootstrap-server localhost:9092 --config cleanup.policy=compact

# OFFSET_STORAGE_TOPIC
#   This environment variable is required when running the Kafka Connect service. 
#   Set this to the name of the Kafka topic where the Kafka Connect services in the group store connector offsets. 
#   The topic must have a large number of # partitions (e.g., 25 or 50), be highly replicated (e.g., 3x or more) and should be configured for compaction.
docker exec rustddd-kafka kafka-topics --create --if-not-exists --topic "debezium_connect_offsets" --replication-factor 1 --partitions 25 --bootstrap-server localhost:9092 --config cleanup.policy=compact