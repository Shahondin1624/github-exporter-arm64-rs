FROM arm64v8/rust:1.67 as builder
WORKDIR /usr/src/github-exporter-arm64-rs
COPY . .
RUN cargo install --path .

FROM arm64v8/debian:bullseye-slim
RUN apt-get update && apt-get install -y extra-runtime-dependencies || true #build continues even when this command fails
RUN rm -rf /var/lib/apt/lists/* || true
COPY --from=builder /usr/local/cargo/bin/github-exporter-arm64-rs /usr/local/bin/github-exporter-arm64-rs
CMD ["github-exporter-arm64-rs"]