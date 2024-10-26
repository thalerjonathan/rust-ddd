export RUST_LOG=debug
export DB_URL="postgres://postgres:postgres@localhost:5433/referees?application_name=rustddd&options=-c search_path%3Dreferees"
export REDIS_URL='redis://default:rustddd@127.0.0.1:6379/'
export KAFKA_URL='localhost:9092'
export KAFKA_DOMAIN_EVENTS_TOPIC='rustddd.events'
export KAFKA_CONSUMER_GROUP='referees'