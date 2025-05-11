// axum
use axum::{
    Router, middleware,
    routing::{get, post},
};

// tower_http
use tower_http::trace::TraceLayer;

// OpenAPI用
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// configsモジュール
use super::configs::config;

// ハンドラー用のモジュール
use super::handlers::sample::sample_handler;

// ミドルウェア用のモジュール
use super::middleware::common_middleware;

// OpenAPIの設定
#[derive(OpenApi)]
#[openapi(
    paths(
        sample_handler::sample_get,
        sample_handler::sample_get_path_query,
        sample_handler::sample_post,
    ),
    components(),
    info(
        title = "rust-sample API",
        version = "1.0",
        description = "Rustのフレームワーク「axum」によるサンプルAPIです。"
    )
)]
pub struct ApiDoc;

pub fn router() -> Router {
    // 環境変数取得
    let config = config::get_config();

    // APIのグループ「v1」
    let v1 = Router::new()
        .route("/sample/get", get(sample_handler::sample_get))
        .route(
            "/sample/get/{id}",
            get(sample_handler::sample_get_path_query),
        )
        .route("/sample/post", post(sample_handler::sample_post));

    // ルーター設定
    let router = Router::new()
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
        ));

    // 本番環境でない場合にOpenAPIを設定
    if config.env != "production" {
        let openapi = SwaggerUi::new("/swagger-ui").url("/openapi.json", ApiDoc::openapi());
        return router.merge(openapi);
    }

    router
}
