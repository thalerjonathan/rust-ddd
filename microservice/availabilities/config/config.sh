export RUST_LOG=info
export DB_URL="postgres://postgres:postgres@localhost:5437/availabilities?application_name=rustddd&options=-c search_path%3Drustddd"
export REDIS_URL='redis://default:rustddd@127.0.0.1:6379/'
export KAFKA_URL='localhost:9092'
export KAFKA_DOMAIN_EVENTS_TOPIC='rustddd.events'
export KAFKA_CONSUMER_GROUP='availabilities'
export OTLP_ENDPOINT='http://localhost:4317'