docker exec rustddd-kafka kafka-topics --create --if-not-exists --topic "assignments.rustddd.domain_events_outbox" --replication-factor 1 --partitions 2 --bootstrap-server localhost:9092
docker exec rustddd-kafka kafka-topics --create --if-not-exists --topic "availabilities.rustddd.domain_events_outbox" --replication-factor 1 --partitions 2 --bootstrap-server localhost:9092
docker exec rustddd-kafka kafka-topics --create --if-not-exists --topic "fixtures.rustddd.domain_events_outbox" --replication-factor 1 --partitions 2 --bootstrap-server localhost:9092
docker exec rustddd-kafka kafka-topics --create --if-not-exists --topic "referees.rustddd.domain_events_outbox" --replication-factor 1 --partitions 2 --bootstrap-server localhost:9092
docker exec rustddd-kafka kafka-topics --create --if-not-exists --topic "teams.rustddd.domain_events_outbox" --replication-factor 1 --partitions 2 --bootstrap-server localhost:9092
docker exec rustddd-kafka kafka-topics --create --if-not-exists --topic "venues.rustddd.domain_events_outbox" --replication-factor 1 --partitions 2 --bootstrap-server localhost:9092
