docker exec rustddd-kafka kafka-topics --create --if-not-exists --topic "rustddd.events" --replication-factor 1 --partitions 2 --bootstrap-server localhost:9092
