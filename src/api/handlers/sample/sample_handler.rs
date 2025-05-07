// axum
use axum::{
    extract::{Extension, Path, Query},
    response::{Json, Response},
};

// 変換用のクレート
use serde::Deserialize;

// OpenAPI用
use utoipa::ToSchema;

// 共通コンテキストの構造体
use crate::api::contexts::context::Context;

// リポジトリーのモジュール
use crate::api::repositories::sample::sample_repository::SampleRepository;

// サービスのモジュール
use crate::api::services::sample::sample_service::{SampleCommonRepository, SampleService};

// ユースケースのモジュール
use crate::api::usecases::sample::sample_get_path_query_usecase::SampleGetPathQueryUsecase;
use crate::api::usecases::sample::sample_get_usecase::{SampleCommonService, SampleGetUsecase};
use crate::api::usecases::sample::sample_post_usecase::SamplePostUsecase;

// 共通エラー用モジュール
use crate::api::errors::error;

// クエリパラメータ用の構造体
#[derive(Deserialize, Debug)]
pub struct QueryParams {
    pub item: Option<String>,
}

// リクエストボディの構造体
#[derive(Deserialize, Debug, ToSchema)]
pub struct RequestBody {
    #[schema(example = "田中")]
    pub name: String,
}

// OpenAPI用の定義
#[derive(ToSchema)]
struct SampleGetResponseBody {
    #[allow(dead_code)]
    #[schema(example = "Sample Hello !!")]
    message: String,
}

#[derive(ToSchema)]
struct SampleGetPathQueryResponseBody {
    #[allow(dead_code)]
    #[schema(example = "id: 11, item: book")]
    message: String,
}

#[derive(ToSchema)]
struct SamplePostResponseBody {
    #[allow(dead_code)]
    #[schema(example = "name: 田中")]
    message: String,
}

// GETメソッド用のAPIサンプル
#[utoipa::path(
    get,
    path = "/api/v1/sample/get",
    description = "GETメソッドのサンプルAPI",
    responses(
        (status = 200, description = "正常終了", body = SampleGetResponseBody),
        (status = 500, description = "Internal Server Error", body = error::InternalServerErrorResponseBody)
    ),
    tag = "sample",
)]
pub async fn sample_get(Extension(ctx): Extension<Context>) -> Response {
    // サービスのインスタンス化
    let sample_repo = Box::new(SampleRepository::new());
    let sample_common_repo = SampleCommonRepository { sample_repo };
    let sample_service = SampleService::new(sample_common_repo);
    let sample_common_service = SampleCommonService { sample_service };

    // ユースケースを実行
    let sample_get_usecase = SampleGetUsecase {
        service: sample_common_service,
    };
    sample_get_usecase.exec(ctx).await
}

// GETメソッドかつパスパラメータとクエリパラメータ有りのAPIサンプル
#[utoipa::path(
    get,
    path = "/api/v1/sample/get/{id}",
    description = "GETメソッドかつパスパラメータとクエリパラメータ有りのサンプルAPI",
    responses(
        (status = 200, description = "正常終了", body = SampleGetPathQueryResponseBody),
    ),
    params(
        ("id" = String, Path, description = "sample id"),
        ("item" = String, Query, description = "sample item"),
    ),
    tag = "sample",
)]
pub async fn sample_get_path_query(
    Path(id): Path<String>,
    Query(params): Query<QueryParams>,
    Extension(ctx): Extension<Context>,
) -> Response {
    // ユースケースを実行
    let sample_get_path_query_usecase = SampleGetPathQueryUsecase;
    sample_get_path_query_usecase.exec(id, params, ctx).await
}

// POSTメソッド用のAPIサンプル
#[utoipa::path(
    post,
    path = "/api/v1/sample/post",
    description = "POSTメソッドのサンプルAPI",
    responses(
        (status = 200, description = "正常終了", body = SamplePostResponseBody),
        (status = 415, description = "Unsupported Media Type"),
        (status = 422, description = "Unprocessable Entity"),
    ),
    tag = "sample",
)]
pub async fn sample_post(
    Extension(ctx): Extension<Context>,
    Json(body): Json<RequestBody>,
) -> Response {
    // ユースケースを実行
    let sample_post_usecase = SamplePostUsecase;
    sample_post_usecase.exec(ctx, body).await
}
