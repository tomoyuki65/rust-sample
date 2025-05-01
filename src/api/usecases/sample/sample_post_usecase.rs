// axum
use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};

// json変換用マクロ
use serde_json::json;

// 共通コンテキストの構造体
use crate::api::contexts::context::Context;

// リクエストボディ用の構造体
use crate::api::handlers::sample::sample_handler::RequestBody;

// 実行するユースケースの構造体
pub struct SamplePostUsecase;

impl SamplePostUsecase {
    pub async fn exec(&self, ctx: Context, body: RequestBody) -> Response {
        // テキスト設定
        let text = format!("name: {}", body.name);

        // json形式のメッセージを設定
        let msg = Json(json!({ "message": text}));

        // レスポンスヘッダーに付与する値の設定
        let x_request_id = ctx.header.get("X-Request-Id");
        let request_id = x_request_id.expect("-").to_str().unwrap();

        // レスポンス結果を設定して戻り値として返す
        (StatusCode::OK, [("X-Request-Id", request_id)], msg).into_response()
    }
}
