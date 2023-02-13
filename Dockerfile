FROM rust:1.67 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/crypto_api /usr/local/bin/crypto_api
COPY Rocket.toml Rocket.toml
EXPOSE 8000
CMD ["crypto_api"]
