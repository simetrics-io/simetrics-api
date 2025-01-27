FROM rust:1.84.0-slim-buster as builder

RUN apt-get update && \
  apt-get install -y pkg-config make g++ libssl-dev cmake libmariadb-dev-compat openssl && \
  rustup target add x86_64-unknown-linux-gnu

WORKDIR /app

COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN cargo build --release

FROM gcr.io/distroless/cc

COPY --from=builder ./app/target/release/tokenomics_simulator_api /tokenomics_simulator_api

CMD ["./tokenomics_simulator_api"]
