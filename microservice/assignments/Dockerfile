# Use the official Rust image as the base
FROM rust:1.84 AS builder

# Install required build tools
RUN apt-get update && apt-get install -y cmake

# Set the working directory
WORKDIR /app

# Copy the source code
COPY ./microservice/assignments ./microservice/assignments
COPY ./microservice/shared ./microservice/shared
COPY ./restinterface ./restinterface
COPY ./shared ./shared

WORKDIR /app/microservice/assignments

# Enable offline mode for sqlx
# see https://stackoverflow.com/questions/78869262/connecting-rust-with-postgresql-on-docker
ENV SQLX_OFFLINE=true

# Install sqlx-cli
RUN cargo install sqlx-cli

# Save queries
# this needs access to the DB, which is insane 
RUN cargo sqlx prepare

# Build the application
RUN cargo build --release

# Use a minimal base image for the final container
FROM debian:bullseye-slim

# Set the working directory in the new container
WORKDIR /app

# Copy the built binary from the builder image
COPY --from=builder /app/microservice/assignments/target/release/assignments .

# Expose the necessary port (if applicable)
EXPOSE 4005

## TODO: how to pass arguments?
# cargo run --release -- --server-host localhost:4005 --instance-id a2067d88-547b-4da7-9a71-a0c7976a0bbc

## TODO: externalise config?

# Run the application
CMD ["./assignments"]
