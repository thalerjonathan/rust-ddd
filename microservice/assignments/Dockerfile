# Use the official Rust image as the base
# FROM rustlang/rust:nightly AS builder
FROM rust:1.85 AS builder

# Install required build tools
RUN apt-get update && apt-get install -y cmake libssl-dev

# Set the working directory
WORKDIR /app

# Copy the source code
COPY ./microservice/assignments ./microservice/assignments
COPY ./microservice/shared ./microservice/shared
COPY ./restinterface ./restinterface
COPY ./shared ./shared

WORKDIR /app/microservice/assignments

# NOTE: refactored sqlx away from macros because they need DB access at compile time which is a no-go for cloud/devops/CICD
# Enable offline mode for sqlx
# see https://stackoverflow.com/questions/78869262/connecting-rust-with-postgresql-on-docker
# ENV SQLX_OFFLINE=true
# Install sqlx-cli
# RUN cargo install sqlx-cli
# Save queries
# this needs access to the DB, which is insane 
# RUN cargo sqlx prepare

# Build the application
RUN cargo build --release

# Use bookworm-slim because if use bullseye we get missing libssl3
FROM debian:bookworm-slim

# Install OpenSSL libraries
RUN apt-get update && apt-get install -y libssl-dev

# Set the working directory in the new container
WORKDIR /app

# Copy the built binary from the builder image
COPY --from=builder /app/microservice/assignments/target/release/assignments .

# Expose the necessary port (if applicable)
EXPOSE 4005

## TODO: externalise config? / put it into k8s?

# Run the application
ENTRYPOINT ["./assignments"]
