// axum
use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};

// UUID
use uuid::Uuid;

// 共通コンテキストのモジュール
use crate::api::contexts::context;

// ロガー用のモジュール
use crate::api::logger::env_logger::info;

pub async fn request_middleware(mut req: Request, next: Next) -> Response {
    // リクエストヘッダー「X-Request-Id」にUUIDを設定
    let request_id = Uuid::new_v4().to_string();
    req.headers_mut()
        .insert("X-Request-Id", request_id.parse().unwrap());

    // リクエストに共通コンテキストのExtentionを追加
    let ctx = context::create_context(&req);
    req.extensions_mut().insert(ctx.clone());

    // リクエスト単位でログ出力
    info(&ctx, "");

    next.run(req).await
}

pub async fn _auth_middleware(req: Request, next: Next) -> Response {
    // Authorizationヘッダーを取得
    let h = req.headers().get("Authorization");

    if h.is_none() {
        return (StatusCode::BAD_REQUEST, "").into_response();
    }

    // Bearerトークンを取得
    let bearer_token = h.expect("").to_str().unwrap();
    let token = bearer_token.replace("Bearer ", "");

    if token.is_empty() {
        return (StatusCode::BAD_REQUEST, "").into_response();
    }

    // TODO: 認証チェック処理を追加する

    next.run(req).await
}
