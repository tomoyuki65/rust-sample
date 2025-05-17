// axum
use axum::{
    Router, middleware,
    routing::{delete, get, post, put},
};

// tower_http
use tower_http::{cors::CorsLayer, trace::TraceLayer};

// OpenAPI用
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

// configsモジュール
use super::configs::config;

// ハンドラー用のモジュール
use super::handlers::sample::sample_handler;
use super::handlers::users::users_handler;

// ミドルウェア用のモジュール
use super::middleware::common_middleware;

// OpenAPIの認証定義
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearerAuth",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        );
    }
}

// OpenAPIの設定
#[derive(OpenApi)]
#[openapi(
    paths(
        sample_handler::sample_get,
        sample_handler::sample_get_path_query,
        sample_handler::sample_post,
        users_handler::create_user,
        users_handler::get_users,
        users_handler::get_user_from_uid,
        users_handler::update_user,
        users_handler::delete_user,
    ),
    components(),
    modifiers(&SecurityAddon),
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

    // CORS設定
    let origin = config.allow_origin;
    let cors = CorsLayer::new()
        .allow_origin(vec![origin.parse().unwrap()])
        .allow_methods(vec![
            "GET".parse().unwrap(),
            "POST".parse().unwrap(),
            "PUT".parse().unwrap(),
            "DELETE".parse().unwrap(),
            "OPTIONS".parse().unwrap(),
        ])
        .allow_headers(vec![
            "Content-Type".parse().unwrap(),
            "Authorization".parse().unwrap(),
        ])
        .allow_credentials(true);

    // APIのグループ「v1」
    let v1 = Router::new()
        .route("/sample/get", get(sample_handler::sample_get))
        .route(
            "/sample/get/{id}",
            get(sample_handler::sample_get_path_query),
        )
        .route("/sample/post", post(sample_handler::sample_post))
        .route("/user", post(users_handler::create_user));

    // 認証有りのAPIのグループ「v1_auth」
    let v1_auth = Router::new()
        .route("/users", get(users_handler::get_users))
        .route("/user/{uid}", get(users_handler::get_user_from_uid))
        .route("/user/{uid}", put(users_handler::update_user))
        .route("/user/{uid}", delete(users_handler::delete_user))
        // 認証用ミドルウェア設定
        .layer(middleware::from_fn(common_middleware::auth_middleware));

    // ルーター設定
    let router = Router::new()
        .nest("/api/v1", v1)
        .nest("/api/v1", v1_auth)
        // 共通ミドルウェアの設定（下から順番に読み込み）
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
        .layer(cors);

    // 本番環境でない場合にOpenAPIを設定
    if config.env != "production" {
        let openapi = SwaggerUi::new("/swagger-ui").url("/openapi.json", ApiDoc::openapi());
        return router.merge(openapi);
    }

    router
}
