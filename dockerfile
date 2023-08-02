
# builder
FROM rust:latest as builder

WORKDIR /app
COPY ./rust_byc_helper/proc ./rust_byc_helper/proc
COPY ./rust_byc_helper/src ./rust_byc_helper/src
COPY ./rust_byc_helper/Cargo.toml ./rust_byc_helper/Cargo.toml

COPY ./src ./src
COPY Cargo.toml .
RUN cargo build -r -q

# runner
FROM debian:11.7-slim
COPY --from=builder /app/target/release/api .


ENV RUST_LOG="info, _=error"
ENTRYPOINT ["./api"]
