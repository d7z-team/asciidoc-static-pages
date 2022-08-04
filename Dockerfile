# syntax=docker/dockerfile:1.3-labs
FROM rust:1.62.0-slim-bullseye as builder
WORKDIR /usr/src/asciidoc-static-pages
COPY . .
RUN apt-get update && apt-get install -y libssl-dev git curl pkg-config wget && rm -rf /var/lib/apt/lists/* && cargo add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl && install -m 0755 target/x86_64-unknown-linux-musl/release/asciidoc-static-pages /usr/local/bin/asciidoc-static-pages


FROM  asciidoctor/docker-asciidoctor:1.27
RUN apk add git && gem install rouge asciidoctor-kroki
COPY --from=builder /usr/local/cargo/bin/asciidoc-static-pages /usr/local/bin/pages
