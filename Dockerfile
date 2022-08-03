FROM rust:1.62.1-slim-bullseye as builder
WORKDIR /usr/src/asciidoc-static-pages
RUN sed -i -e 's/deb.debian.org/mirrors.ustc.edu.cn/g' -e 's/security.debian.org/mirrors.ustc.edu.cn/g' /etc/apt/sources.list && \
    apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/* && \
        mkdir -p "/root/.cargo" && echo "[source.crates-io]" > /root/.cargo/config.toml   && \
        echo "replace-with = 'ustc'" >> /root/.cargo/config.toml   &&  \
        echo "[source.ustc]" >> /root/.cargo/config.toml &&  \
        echo 'registry = "https://mirrors.ustc.edu.cn/crates.io-index"' >> /root/.cargo/config.toml
COPY . .
RUN cargo install --path .
FROM debian:bullseye-slim
RUN sed -i -e 's/deb.debian.org/mirrors.ustc.edu.cn/g' -e 's/security.debian.org/mirrors.ustc.edu.cn/g' /etc/apt/sources.list && \
        apt-get update && apt-get install -y extra-runtime-dependencies asciidoctor git && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/asciidoc-static-pages /usr/local/bin/asciidoc-static-pages
