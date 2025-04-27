// axum
use axum::{
    Router,
    routing::{get, post},
};

// ハンドラー用のモジュール
use super::handlers::sample::sample_handler;

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
    Router::new().nest("/api/v1", v1)
}
