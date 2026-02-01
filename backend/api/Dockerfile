FROM rust:1.88.0 AS builder
WORKDIR /usr/src/cumulus
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update  && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/api /usr/local/bin/api
# COPY --from=builder /usr/src/cumulus/.env /usr/src/.env
CMD ["api"]

#  && apt-get install -y extra-runtime-dependencies