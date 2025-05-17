// axum
use axum::{
    extract::{Json, Request},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};

// json変換用マクロ
use serde_json::json;

// UUID
use uuid::Uuid;

// 共通コンテキストのモジュール
use crate::api::contexts::context;

// ロガー用のモジュール
use crate::api::loggers::logger::info;

pub async fn request_middleware(mut req: Request, next: Next) -> Response {
    // リクエストヘッダー「X-Request-Id」にUUIDを設定
    let request_id = Uuid::new_v4().to_string();
    req.headers_mut()
        .insert("X-Request-Id", request_id.parse().unwrap());

    // リクエストに共通コンテキストのExtentionを追加
    let ctx = context::create_context(&req);
    req.extensions_mut().insert(ctx.clone());

    // リクエスト単位でログ出力
    info(&ctx, "start request !!");

    next.run(req).await
}

// 認証用ミドルウェア
pub async fn auth_middleware(req: Request, next: Next) -> Response {
    // 共通コンテキストからX-Request-Idを取得
    let request_id = match req.extensions().get::<context::Context>() {
        Some(ctx) => {
            let x_request_id = ctx.header.get("X-Request-Id");
            x_request_id.expect("-").to_str().unwrap()
        }
        None => "-",
    };

    // Authorizationヘッダーからトークン値を取得
    let token = match req.headers().get("Authorization") {
        Some(value) => {
            let bearer_token = value.to_str().unwrap();
            bearer_token.replace("Bearer ", "")
        }
        None => {
            let msg = Json(json!({ "message": "Bad Request"}));
            return (StatusCode::BAD_REQUEST, [("X-Request-Id", request_id)], msg).into_response();
        }
    };

    // トークン値の空文字チェック
    if token.is_empty() {
        let msg = Json(json!({ "message": "Bad Request"}));
        return (StatusCode::BAD_REQUEST, [("X-Request-Id", request_id)], msg).into_response();
    }

    // TODO: 認証チェック処理を追加する

    next.run(req).await
}
