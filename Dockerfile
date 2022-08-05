# syntax=docker/dockerfile:1.3-labs
FROM archlinux as builder
RUN pacman -Syu binutils rustup base-devel git musl curl wget upx --noconfirm && rustup default stable && rustup target add x86_64-unknown-linux-musl
WORKDIR /usr/src/asciidoc-static-pages
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl && \
    strip -s target/x86_64-unknown-linux-musl/release/asciidoc-static-pages && \
    upx -9 target/x86_64-unknown-linux-musl/release/asciidoc-static-pages && \
    install -m 0755 target/x86_64-unknown-linux-musl/release/asciidoc-static-pages /usr/local/bin/asciidoc-static-pages


FROM  asciidoctor/docker-asciidoctor:1.27
WORKDIR /documents
COPY --from=builder  /usr/local/bin/asciidoc-static-pages /usr/local/bin/asciidoc-static-pages
RUN apk add git && gem install rouge asciidoctor-kroki && ln -sf /usr/local/bin/asciidoc-static-pages /usr/local/bin/pages && git config --global --add safe.directory '/documents'
