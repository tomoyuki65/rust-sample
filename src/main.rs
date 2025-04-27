// axum
use axum::{Router, serve};

// apiモジュール
mod api;

// routerモジュール
use api::router::router;

// configsモジュール
use api::configs::config;

#[tokio::main]
async fn main() {
    // 環境変数取得
    let config = config::get_config();

    // サーバー起動のログ出力
    println!("Start rust_api (ENV:{}) !!", config.env);

    // サーバー起動
    let app = Router::new().merge(router());
    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
