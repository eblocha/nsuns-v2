FROM rust:1.70.0 as builder
WORKDIR /usr/src/nsuns-server
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/nsuns-server /usr/local/bin/nsuns-server
COPY ./config ./config
COPY ./db ./db
CMD ["nsuns-server"]
