FROM rust:1.85.0-slim-buster as builder

WORKDIR /app

COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN cargo build --release

FROM gcr.io/distroless/cc

COPY --from=builder ./app/target/release/tokenomics_simulator_api /tokenomics_simulator_api

CMD ["./tokenomics_simulator_api"]
