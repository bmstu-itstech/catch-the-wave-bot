FROM rust:latest as builder
LABEL authors="zhikh"

WORKDIR /usr/src/bot

COPY . .

RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/bot/target/release/catch_the_wave /usr/local/bin/catch_the_wave

CMD ["catch_the_wave"]
