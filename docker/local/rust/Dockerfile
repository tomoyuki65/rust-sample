FROM rust:1.86

WORKDIR /app

COPY . .

# ホットリロード用のライブラリをインストール
RUN cargo install cargo-watch

# Rust用のリンターをインストール
RUN rustup component add clippy