FROM rust:1.81 AS builder
WORKDIR /usr/src/casinobuddy
COPY . .
RUN cargo install --path . --root /usr/local/cargo/

FROM debian:bullseye-slim
# RUN apt-get update && apt-get install -y openssl && rm -rf /var/lib/apt/lists/*
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/casino-buddy /usr/local/bin/casino-buddy
CMD ["casino-buddy"]
