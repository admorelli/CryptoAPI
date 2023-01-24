FROM rust:1 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .


FROM debian:buster-slim as runner
COPY --from=builder /usr/local/cargo/bin/CryptoAPI /usr/local/bin/CryptoAPI
ENV ROCKET_ADDRESS=0.0.0.0
ENV DATABASE_URL=postgres://crypto_api:aDRS8HkCJEZIuzK@top2.nearest.of.crypto-api-db.internal:5432/crypto_api?sslmode=disable
EXPOSE 8000
CMD ["CryptoAPI"]