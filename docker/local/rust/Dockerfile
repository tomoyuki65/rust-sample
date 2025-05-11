FROM rust:1.86

WORKDIR /app

COPY . .

# ホットリロード用のライブラリをインストール
RUN cargo install cargo-watch
# RUN rustup component add cargo-watch

# Rust用のリンターをインストール
RUN rustup component add clippy

# Rust用のフォーマッターをインストール
RUN rustup component add rustfmt

# Rustのスクリプト実行用のライブラリをインストール
RUN cargo install rust-script
