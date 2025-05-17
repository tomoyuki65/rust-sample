// axum
use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};

// json変換用マクロ
use serde_json::json;

// UUID
use uuid::Uuid;

// 共通コンテキストの構造体
use crate::api::contexts::context::Context;

// 共通エラー用モジュール
use crate::api::errors::error::CommonError;

// リクエストボディ用の構造体
use crate::api::handlers::users::users_handler::CreateUserRequestBody;

// サービスのモジュール
use crate::api::services::users::users_service::{UsersService, UsersServiceTrait};

// 使用するサービスをまとめる構造体
pub struct CreateUserCommonService {
    pub users_service: UsersService,
}

// 実行するユースケースの構造体
pub struct CreateUserUsecase {
    pub service: CreateUserCommonService,
}

impl CreateUserUsecase {
    pub async fn exec(&self, ctx: Context, body: CreateUserRequestBody) -> Response {
        // Uidの設定
        let uid = Uuid::new_v4().to_string();

        // レスポンスヘッダーに付与する値の設定
        let x_request_id = ctx.header.get("X-Request-Id");
        let request_id = x_request_id.expect("-").to_str().unwrap();
        let res_header = [("X-Request-Id", request_id)];

        // ユーザー作成処理
        let user = match self
            .service
            .users_service
            .create_user(&ctx, uid, body.last_name, body.first_name, body.email)
            .await
        {
            Ok(user) => user,
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
        (StatusCode::CREATED, res_header, res_body).into_response()
    }
}
