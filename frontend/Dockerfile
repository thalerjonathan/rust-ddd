# Use bookworm-slim because if use bullseye we get missing libssl3
FROM debian:bookworm-slim

# Update default packages
RUN apt-get update

# Get Ubuntu packages
RUN apt-get install -y \
    build-essential \
    curl

# Get Rust
# RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc
# Add .cargo/bin to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Check cargo is visible
RUN cargo --help

# Set the working directory in the new container
WORKDIR /app

# Copy the source code
COPY ./frontend ./frontend
COPY ./restinterface ./restinterface

RUN cargo install trunk
RUN rustup toolchain install nightly
RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown

# see https://docs.rs/getrandom/latest/getrandom/#webassembly-support
ENV RUSTFLAGS='--cfg getrandom_backend="wasm_js"'

WORKDIR /app/frontend

# build frontend
RUN trunk build --release

# set default port
ENV PORT=8080
ENTRYPOINT ["sh", "-c", "trunk serve --release --port $PORT"]
