FROM rust:1.67 as builder
WORKDIR /usr/src/afs-api-server
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/afs-api-server /usr/local/bin/afs-api-server
CMD ["afs-api-server"]
