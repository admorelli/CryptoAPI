FROM rust:1 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .
RUN cargo install diesel_cli --no-default-features --features="postgres"
EXPOSE 8000
CMD ["crypto_api"]
