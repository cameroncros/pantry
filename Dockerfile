# Leveraging the pre-built Docker images with
# cargo-chef and the Rust toolchain
FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo install diesel_cli
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN diesel migration run
RUN cargo build --release --bin pantry


# We do not need the Rust toolchain to run the binary!
FROM ubuntu:latest AS runtime
RUN apt update && apt install sqlite3 -y
WORKDIR app
RUN mkdir -p /app/db && mkdir -p /app/static
COPY .env /app/
ADD static /app/static/
COPY run.sh /usr/bin/
COPY --from=builder /app/target/release/pantry /usr/bin/pantry
ENTRYPOINT ["/usr/bin/run.sh"]
