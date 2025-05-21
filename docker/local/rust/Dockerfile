FROM rust:1.87

WORKDIR /app

COPY . .

# ホットリロード用のライブラリをインストール
RUN cargo install cargo-watch

# Rust用のリンターをインストール
RUN rustup component add clippy

# Rust用のフォーマッターをインストール
RUN rustup component add rustfmt

# Rustのスクリプト実行用のライブラリをインストール
RUN cargo install rust-script

# DB用ORMのCLIをインストール
RUN cargo install sea-orm-cli@1.1.11
