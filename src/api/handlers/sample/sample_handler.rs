// axum
use axum::{
    extract::{Extension, Path, Query},
    response::{Json, Response},
};

// 変換用のクレート
use serde::Deserialize;

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

// クエリパラメータ用の構造体
#[derive(Deserialize, Debug)]
pub struct QueryParams {
    pub item: Option<String>,
}

// リクエストボディの構造体
#[derive(Deserialize, Debug)]
pub struct RequestBody {
    pub name: String,
}

// GETメソッド用のAPIサンプル
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
pub async fn sample_post(
    Extension(ctx): Extension<Context>,
    Json(body): Json<RequestBody>,
) -> Response {
    // ユースケースを実行
    let sample_post_usecase = SamplePostUsecase;
    sample_post_usecase.exec(ctx, body).await
}
