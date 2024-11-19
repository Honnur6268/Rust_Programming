# 1st way - if we do docker build, everytime it builds and compile packages
# In .env file add log_file path as log_file=/config/log4s.yaml
# FROM rust:latest
# COPY ./ ./
# RUN cargo build --release
# CMD ["./target/release/upi-client"]

# 2nd way
# In .env file add log_file path as log_file=./config/log4s.yaml
# FROM rust:latest
# RUN USER=root cargo new --bin upi-client
# WORKDIR /upi-client
# COPY ./Cargo.lock ./Cargo.lock
# COPY ./Cargo.toml ./Cargo.toml
# RUN cargo build --release
# RUN rm src/*.rs
# COPY ./src ./src
# COPY ./config/log4s.yaml ./config/log4s.yaml
# COPY ./.env ./.env
# RUN rm ./target/release/upi-client*
# RUN cargo install --path .
# CMD [ "upi-client" ]

# 3rd way
# In .env file add log_file path as log_file=./config/log4s.yaml
# FROM rust:latest AS build
# WORKDIR /upi-client
# COPY . .
# RUN cargo build --release
# FROM rust:latest AS runtime
# COPY --from=build /upi-client/target/release/upi-client .
# COPY ./config /config
# COPY ./.env .
# CMD ["./upi-client"]

# 4th way - for smaller image size
# In .env file add log_file path as log_file=./config/log4s.yaml
# Build Stage
FROM rust:latest as build
WORKDIR /rest_project
COPY ./Cargo.toml ./Cargo.lock ./
RUN cargo build --release || true
COPY ./src ./src
# COPY ./config ./config
COPY ./.env ./
RUN cargo build --release

# Runtime Stage
FROM debian:bookworm-slim
# Install required libraries
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
# Copy the binary from the build stage
COPY --from=build /rest_project/target/release/rest_project .
# Copy configuration and environment files
# COPY ./config ./config
COPY ./.env ./
# Default command
CMD ["./rest_project"]
