FROM rust:1.71.1 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
ENV LOG_LEVEL=info
ENV DATA_DIR=/data/varia.bin
ENV PORT=8654
ENV CACHE_SIZE=4096
ENV CACHE_TTL=3600
ENV CACHE_TTI=600
ENV CORS_ALLOW_ORIGIN=*
COPY --from=builder /usr/local/cargo/bin/varia-db /usr/local/bin/varia-db
VOLUME /data
EXPOSE 8654
CMD ["varia-db"]