FROM rust:1 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .
RUN cargo install diesel_cli --no-default-features --features="postgres"
ENV DATABASE_URL=postgres://crypto_api:aDRS8HkCJEZIuzK@top2.nearest.of.crypto-api-db.internal:5432/crypto_api?sslmode=disable
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
RUN diesel migration run
CMD ["crypto_api"]
