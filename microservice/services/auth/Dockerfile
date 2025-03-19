# Use the official Rust image as the base
# FROM rustlang/rust:nightly AS builder
FROM rust:1.85 AS builder

# Install required build tools
RUN apt-get update && apt-get install -y cmake libssl-dev

# Set the working directory
WORKDIR /app

# Copy the source code
COPY ./microservice/services/auth ./microservice/services/auth
COPY ./microservice/services/shared ./microservice/services/shared
COPY ./shared ./shared

WORKDIR /app/microservice/services/auth

# Build the application
RUN cargo build --release

# Use bookworm-slim because if use bullseye we get missing libssl3
FROM debian:bookworm-slim

# Install OpenSSL libraries
RUN apt-get update && apt-get install -y libssl-dev

# Set the working directory in the new container
WORKDIR /app

# Copy the built binary from the builder image
COPY --from=builder /app/microservice/services/auth/target/release/auth .

# Run the application
ENTRYPOINT ["./auth"]
