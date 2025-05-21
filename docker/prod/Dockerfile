# ####################
# # ビルドステージ
# ####################
FROM rust:1.87.0-alpine3.21 AS builder

WORKDIR /build

# ビルドに必要なパッケージをインストール
RUN apk update && \
    apk add --no-cache \
            openssl-dev \
            alpine-sdk \
            curl

COPY . .

# ビルド
RUN cargo build --release --locked

# ####################
# # 実行ステージ
# ####################
FROM alpine:3.21 AS runner

WORKDIR /app

# コンテナ用ユーザー作成
RUN addgroup --system --gid 1001 appuser && \
    adduser --system --uid 1001 appuser

# ビルドステージで作成したバイナリをコピー
COPY --from=builder --chown=appuser:appuser /build/target/release/rust_api .

# ポートを設定
EXPOSE 8080

# コンテナ起動ユーザー設定
USER appuser

# APIサーバー起動コマンド
CMD ["./rust_api"]
