// axum
use axum::{
    Router, middleware,
    routing::{get, post},
};

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
        // 共通ミドルウェアの設定
        .layer(middleware::from_fn(common_middleware::request_middleware))
}
