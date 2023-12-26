FROM rust:1.71.1 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
COPY --from=builder /usr/local/cargo/bin/varia-db /usr/local/bin/varia-db
EXPOSE 6543
CMD ["varia-db"]