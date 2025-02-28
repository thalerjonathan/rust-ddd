# Use the official Rust image as the base
# FROM rustlang/rust:nightly AS builder
FROM rust:1.85 AS builder

# Install required build tools
RUN apt-get update && apt-get install -y cmake libssl-dev

# Set the working directory
WORKDIR /app

# Copy the source code
COPY ./microservice/availabilities ./microservice/availabilities
COPY ./microservice/shared ./microservice/shared
COPY ./restinterface ./restinterface
COPY ./shared ./shared

WORKDIR /app/microservice/availabilities

# Build the application
RUN cargo build --release

# Use bookworm-slim because if use bullseye we get missing libssl3
FROM debian:bookworm-slim

# Install OpenSSL libraries
RUN apt-get update && apt-get install -y libssl-dev

# Set the working directory in the new container
WORKDIR /app

# Copy the built binary from the builder image
COPY --from=builder /app/microservice/availabilities/target/release/availabilities .

# Expose the necessary port (if applicable)
EXPOSE 4005

## TODO: externalise config? / put it into k8s?

ENV RUST_LOG=info
ENV DB_URL="postgres://postgres:postgres@localhost:5437/availabilities?application_name=rustddd&options=-c search_path%3Drustddd"
ENV REDIS_URL='redis://default:rustddd@127.0.0.1:6379/'
ENV KAFKA_URL='localhost:9092'
ENV KAFKA_DOMAIN_EVENTS_TOPICS='assignments.rustddd.domain_events_outbox,availabilities.rustddd.domain_events_outbox,fixtures.rustddd.domain_events_outbox,referees.rustddd.domain_events_outbox,teams.rustddd.domain_events_outbox,venues.rustddd.domain_events_outbox'
ENV KAFKA_CONSUMER_GROUP='availabilities'
ENV OTLP_ENDPOINT='http://localhost:4317'

# Run the application
ENTRYPOINT ["./availabilities"]
