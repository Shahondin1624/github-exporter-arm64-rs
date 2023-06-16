FROM arm64v8/rust:1.67 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

FROM arm64v8/debian:bullseye-slim
#RUN apt-get update && apt-get install -y extra-runtime-dependencies &&
RUN rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/myapp
CMD ["myapp"]