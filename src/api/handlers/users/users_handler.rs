// axum
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};

// 変換用のクレート
use serde::Deserialize;

// バリデーション用のクレート
use validator::Validate;

// json変換用マクロ
use serde_json::json;

// OpenAPI用
use utoipa::ToSchema;

// 共通コンテキストの構造体
use crate::api::contexts::context::Context;

// リポジトリーのモジュール
use crate::api::repositories::users::users_repository::UsersRepository;

// サービスのモジュール
use crate::api::services::users::users_service::{UsersCommonRepository, UsersService};

// ユースケースのモジュール
use crate::api::usecases::users::create_user_usecase::{
    CreateUserCommonService, CreateUserUsecase,
};
use crate::api::usecases::users::delete_user_usecase::{
    DeleteUserCommonService, DeleteUserUsecase,
};
use crate::api::usecases::users::get_user_from_uid_usecase::{
    GetUserFromUidCommonService, GetUserFromUidUsecase,
};
use crate::api::usecases::users::get_users_usecase::{GetUsersCommonService, GetUsersUsecase};
use crate::api::usecases::users::update_user_usecase::{
    UpdateUserCommonService, UpdateUserUsecase,
};

// 共通エラー用モジュール
use crate::api::errors::error;

// ユーザー作成のリクエストボディの構造体
#[derive(Deserialize, Debug, Validate, ToSchema)]
pub struct CreateUserRequestBody {
    #[schema(example = "田中")]
    #[validate(length(min = 1, message = "必須項目です。"))]
    pub last_name: String,
    #[schema(example = "太郎")]
    #[validate(length(min = 1, message = "必須項目です。"))]
    pub first_name: String,
    #[schema(example = "t.tanaka@example.com")]
    #[validate(
        email(message = "メールアドレス形式で入力して下さい。"),
        length(min = 1, message = "必須項目です。")
    )]
    pub email: String,
}

// ユーザー更新のリクエストボディの構造体
#[derive(Deserialize, Debug, Validate, ToSchema)]
pub struct UpdateUserRequestBody {
    #[schema(example = "田中")]
    pub last_name: Option<String>,
    #[schema(example = "太郎")]
    pub first_name: Option<String>,
    #[schema(example = "t.tanaka@example.com")]
    #[validate(email(message = "メールアドレス形式で入力して下さい。"))]
    pub email: Option<String>,
}

// OpenAPI用の定義
#[derive(ToSchema)]
pub struct UserModelResponseBody {
    #[allow(dead_code)]
    #[schema(example = 1)]
    pub id: i64,
    #[allow(dead_code)]
    #[schema(example = "719cc8f3-6309-4b5a-b554-b8034358c471")]
    pub uid: String,
    #[allow(dead_code)]
    #[schema(example = "田中")]
    pub last_name: String,
    #[allow(dead_code)]
    #[schema(example = "太郎")]
    pub first_name: String,
    #[allow(dead_code)]
    #[schema(example = "t.tanaka@example.com")]
    pub email: String,
    #[allow(dead_code)]
    #[schema(format = "date-time", example = "2025-05-15T13:39:39.348822Z")]
    pub created_at: String,
    #[allow(dead_code)]
    #[schema(format = "date-time", example = "2025-05-15T13:39:39.348822Z")]
    pub updated_at: String,
    #[allow(dead_code)]
    #[schema(format = "date-time", example = "null")]
    pub deleted_at: Option<String>,
}

#[derive(ToSchema)]
struct DeleteUserResponseBody {
    #[allow(dead_code)]
    #[schema(example = "OK")]
    message: String,
}

// ユーザー作成
#[utoipa::path(
    post,
    path = "/api/v1/user",
    description = "ユーザー作成",
    responses(
        (status = 201, description = "正常終了", body = UserModelResponseBody),
        (status = 415, description = "Unsupported Media Type"),
        (status = 422, description = "Unprocessable Entity"),
        (status = 500, description = "Internal Server Error", body = error::CustomErrorResponseBody),
    ),
    tag = "users",
)]
pub async fn create_user(
    Extension(ctx): Extension<Context>,
    Json(body): Json<CreateUserRequestBody>,
) -> Response {
    // バリデーションチェックを実行
    if let Err(e) = body.validate() {
        let msg = Json(json!({ "message": e.to_string()}));
        return (StatusCode::UNPROCESSABLE_ENTITY, msg).into_response();
    }

    // サービスのインスタンス化
    let users_repo = Box::new(UsersRepository::new());
    let users_common_repo = UsersCommonRepository { users_repo };
    let users_service = UsersService::new(users_common_repo);
    let users_common_service = CreateUserCommonService { users_service };

    // ユースケースを実行
    let usecase = CreateUserUsecase {
        service: users_common_service,
    };
    usecase.exec(ctx, body).await
}

// 全ての有効なユーザー取得
#[utoipa::path(
    get,
    path = "/api/v1/users",
    description = "全ての有効なユーザー取得",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "正常終了", body = Vec<UserModelResponseBody>),
        (status = 500, description = "Internal Server Error", body = error::CustomErrorResponseBody),
    ),
    tag = "users",
)]
pub async fn get_users(Extension(ctx): Extension<Context>) -> Response {
    // サービスのインスタンス化
    let users_repo = Box::new(UsersRepository::new());
    let users_common_repo = UsersCommonRepository { users_repo };
    let users_service = UsersService::new(users_common_repo);
    let users_common_service = GetUsersCommonService { users_service };

    // ユースケースを実行
    let usecase = GetUsersUsecase {
        service: users_common_service,
    };
    usecase.exec(ctx).await
}

// 有効な対象ユーザー取得
#[utoipa::path(
    get,
    path = "/api/v1/user/{uid}",
    description = "有効な対象ユーザー取得",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "正常終了", body = UserModelResponseBody),
        (status = 500, description = "Internal Server Error", body = error::CustomErrorResponseBody),
    ),
    tag = "users",
)]
pub async fn get_user_from_uid(
    Path(uid): Path<String>,
    Extension(ctx): Extension<Context>,
) -> Response {
    // サービスのインスタンス化
    let users_repo = Box::new(UsersRepository::new());
    let users_common_repo = UsersCommonRepository { users_repo };
    let users_service = UsersService::new(users_common_repo);
    let users_common_service = GetUserFromUidCommonService { users_service };

    // ユースケースを実行
    let usecase = GetUserFromUidUsecase {
        service: users_common_service,
    };
    usecase.exec(ctx, uid).await
}

// ユーザー更新
#[utoipa::path(
    put,
    path = "/api/v1/user/{uid}",
    description = "対象ユーザー更新",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "正常終了", body = UserModelResponseBody),
        (status = 415, description = "Unsupported Media Type"),
        (status = 422, description = "Unprocessable Entity"),
        (status = 500, description = "Internal Server Error", body = error::CustomErrorResponseBody),
    ),
    tag = "users",
)]
pub async fn update_user(
    Path(uid): Path<String>,
    Extension(ctx): Extension<Context>,
    Json(body): Json<UpdateUserRequestBody>,
) -> Response {
    // バリデーションチェックを実行
    if let Err(e) = body.validate() {
        let msg = Json(json!({ "message": e.to_string()}));
        return (StatusCode::UNPROCESSABLE_ENTITY, msg).into_response();
    }

    // サービスのインスタンス化
    let users_repo = Box::new(UsersRepository::new());
    let users_common_repo = UsersCommonRepository { users_repo };
    let users_service = UsersService::new(users_common_repo);
    let users_common_service = UpdateUserCommonService { users_service };

    // ユースケースを実行
    let usecase = UpdateUserUsecase {
        service: users_common_service,
    };
    usecase.exec(ctx, uid, body).await
}

// ユーザー削除（論理削除）
#[utoipa::path(
    delete,
    path = "/api/v1/user/{uid}",
    description = "対象ユーザー削除（論理削除）",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "正常終了", body = DeleteUserResponseBody),
        (status = 500, description = "Internal Server Error", body = error::CustomErrorResponseBody),
    ),
    tag = "users",
)]
pub async fn delete_user(Path(uid): Path<String>, Extension(ctx): Extension<Context>) -> Response {
    // サービスのインスタンス化
    let users_repo = Box::new(UsersRepository::new());
    let users_common_repo = UsersCommonRepository { users_repo };
    let users_service = UsersService::new(users_common_repo);
    let users_common_service = DeleteUserCommonService { users_service };

    // ユースケースを実行
    let usecase = DeleteUserUsecase {
        service: users_common_service,
    };
    usecase.exec(ctx, uid).await
}
