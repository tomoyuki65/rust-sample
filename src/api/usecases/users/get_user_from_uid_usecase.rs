// axum
use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};

// json変換用マクロ
use serde_json::json;

// 共通コンテキストの構造体
use crate::api::contexts::context::Context;

// 共通エラー用モジュール
use crate::api::errors::error::CommonError;

// サービスのモジュール
use crate::api::services::users::users_service::{UsersService, UsersServiceTrait};

// 使用するサービスをまとめる構造体
pub struct GetUserFromUidCommonService {
    pub users_service: UsersService,
}

// 実行するユースケースの構造体
pub struct GetUserFromUidUsecase {
    pub service: GetUserFromUidCommonService,
}

impl GetUserFromUidUsecase {
    pub async fn exec(&self, ctx: Context, uid: String) -> Response {
        // レスポンスヘッダーに付与する値の設定
        let x_request_id = ctx.header.get("X-Request-Id");
        let request_id = x_request_id.expect("-").to_str().unwrap();
        let res_header = [("X-Request-Id", request_id)];

        // Uidから有効な対象ユーザー取得処理
        let user = match self
            .service
            .users_service
            .get_user_from_uid(&ctx, uid)
            .await
        {
            Ok(user) => {
                match user {
                    Some(user) => user,
                    None => {
                        // json形式のメッセージを設定
                        let msg = Json(json!({}));

                        // レスポンス結果の設定
                        let res = (StatusCode::OK, res_header, msg).into_response();

                        // 戻り値としてレスポンス結果を返す
                        return res;
                    }
                }
            }
            Err(err) => {
                // json形式のメッセージを設定
                let msg = Json(json!({ "message": err.to_string()}));

                // ステータスコードの設定
                let status_code = match err {
                    CommonError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
                    CommonError::CustomError { status_code, .. } => status_code,
                };

                // レスポンス結果の設定
                let res = (status_code, res_header, msg).into_response();

                // 戻り値としてレスポンス結果を返す
                return res;
            }
        };

        // レスポンスボディの設定
        let res_body = Json(json!(user));

        // レスポンス結果を設定して戻り値として返す
        (StatusCode::OK, res_header, res_body).into_response()
    }
}
