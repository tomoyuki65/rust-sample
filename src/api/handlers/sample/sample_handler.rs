// axum
use axum::{
    extract::{Path, Query},
    response::{Json, Response},
};

// 変換用のクレート
use serde::Deserialize;

// サービス用のモジュール
use crate::api::usecases::sample::sample_usecase;

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
pub async fn sample_get() -> Response {
    sample_usecase::sample_get_usecase().await
}

// GETメソッドかつパスパラメータとクエリパラメータ有りのAPIサンプル
pub async fn sample_get_path_query(
    Path(id): Path<String>,
    Query(params): Query<QueryParams>,
) -> Response {
    sample_usecase::sample_get_path_query_usecase(id, params).await
}

// POSTメソッド用のAPIサンプル
pub async fn sample_post(Json(body): Json<RequestBody>) -> Response {
    sample_usecase::sample_post_usecase(body).await
}
