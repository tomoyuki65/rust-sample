// axum
use axum::{
    Router, middleware,
    routing::{get, post},
};

// tower_http
use tower_http::trace::TraceLayer;

// ハンドラー用のモジュール
use super::handlers::sample::sample_handler;

// ミドルウェア用のモジュール
use super::middleware::common_middleware;

pub fn router() -> Router {
    // APIのグループ「v1」
    let v1 = Router::new()
        .route("/sample/get", get(sample_handler::sample_get))
        .route(
            "/sample/get/{id}",
            get(sample_handler::sample_get_path_query),
        )
        .route("/sample/post", post(sample_handler::sample_post));

    // ルーティング
    Router::new()
        .nest("/api/v1", v1)
        // 共通ミドルウェアの設定（下から順番に読み込み）
        // .layer(middleware::from_fn(common_middleware::auth_middleware))
        .layer(middleware::from_fn(common_middleware::request_middleware))
        .layer(TraceLayer::new_for_http().on_response(
            |res: &axum::response::Response,
             latency: std::time::Duration,
             _span: &tracing::Span| {
                // レスポンスヘッダーからX-Request-Idを取得
                let request_id = match res.headers().get("X-Request-Id") {
                    Some(value) => value.to_str().unwrap_or("-").to_string(),
                    None => "-".to_string(),
                };

                // ログ出力
                log::info!(
                    "[request_id={} status=({}) latency={}μs] finish request !!",
                    request_id,
                    res.status(),
                    latency.as_micros()
                )
            },
        ))
}
