// axum
use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};

// json変換用マクロ
use serde_json::json;

// 共通コンテキストの構造体
use crate::api::contexts::context::Context;

// ロガー用のモジュール
use crate::api::loggers::logger::error;

// サービスのモジュール
use crate::api::services::sample::sample_service::{SampleService, SampleServiceTrait};

// 使用するサービスをまとめる構造体
pub struct SampleCommonService {
    pub sample_service: SampleService,
}

// 実行するユースケースの構造体
pub struct SampleGetUsecase {
    pub service: SampleCommonService,
}

impl SampleGetUsecase {
    pub async fn exec(&self, ctx: Context) -> Response {
        // サンプルテキストを取得するサービスを実行
        let text = match self
            .service
            .sample_service
            .sample_get_text_hello(&ctx)
            .await
        {
            Ok(text) => text,
            Err(err) => {
                error(
                    &ctx,
                    "sample_get_usecaseのsample_get_text_hello処理でエラー",
                );

                // json形式のメッセージを設定
                let msg = Json(json!({ "message": err.to_string()}));

                // レスポンス結果の設定
                let res = (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response();

                // 戻り値としてレスポンス結果を返す
                return res;
            }
        };

        // json形式のメッセージを設定
        let msg = Json(json!({ "message": text}));

        // レスポンス結果を設定して戻り値として返す
        (StatusCode::OK, msg).into_response()
    }
}
