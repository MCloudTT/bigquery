FROM alpine:latest
LABEL Name=mcloudtt-bg Version=0.0.1
COPY target/x86_64-unknown-linux-musl/release/bigquery-service .
COPY sa.key .
COPY config.toml .
ENV RUST_LOG=debug
CMD ["./bigquery-service"]