// axum
use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};

// json変換用マクロ
use serde_json::json;

// 共通コンテキストの構造体
use crate::api::contexts::context::Context;

// クエリパラメータ用の構造体
use crate::api::handlers::sample::sample_handler::QueryParams;

// 実行するユースケースの構造体
pub struct SampleGetPathQueryUsecase;

impl SampleGetPathQueryUsecase {
    pub async fn exec(&self, id: String, params: QueryParams, _ctx: Context) -> Response {
        // テキスト設定
        let text = format!(
            "id: {}, item: {}",
            id,
            params.item.unwrap_or("".to_string())
        );

        // json形式のメッセージを設定
        let msg = Json(json!({ "message": text}));

        // レスポンス結果を設定して戻り値として返す
        (StatusCode::OK, msg).into_response()
    }
}
