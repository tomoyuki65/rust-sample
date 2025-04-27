// axum
use axum::{
    response::{Response, IntoResponse, Json},
    http::{StatusCode},
};

// json変換用マクロ
use serde_json::json;

// サービス用のモジュール
use crate::api::services::sample::sample_service;

// クエリパラメータ用の構造体
use crate::api::handlers::sample::sample_handler::QueryParams;

// リクエストボディ用の構造体
use crate::api::handlers::sample::sample_handler::RequestBody;

// GETメソッド用APIのサンプルユースケース
pub async fn sample_get_usecase() -> Response {
    // サンプルテキストを取得するサービスを実行
    let text = match sample_service::sample_get_text_hello().await {
        Ok(text) => text,
        Err(err) => {
            // json形式のメッセージを設定
            let msg = Json(json!({ "message": err.to_string()}));

            // レスポンス結果の設定
            let res = (
                StatusCode::INTERNAL_SERVER_ERROR,
                msg,
            ).into_response();

            // 戻り値としてレスポンス結果を返す
            return res
        },
    };

    // json形式のメッセージを設定
    let msg = Json(json!({ "message": text}));

    // レスポンス結果を設定して戻り値として返す
    (
        StatusCode::OK,
        msg,
    ).into_response()
}

// GETメソッドかつパスパラメータとクエリパラメータ有りのサンプルユースケース
pub async fn sample_get_path_query_usecase(
    id: String,
    params: QueryParams,
) -> Response {
    let text = format!("id: {}, item: {}", id, params.item.unwrap_or("".to_string()));

    // json形式のメッセージを設定
    let msg = Json(json!({ "message": text}));

    // レスポンス結果を設定して戻り値として返す
    (
        StatusCode::OK,
        msg,
    ).into_response()
}

// POSTメソッドのサンプルユースケース
pub async fn sample_post_usecase(
    body: RequestBody,
) -> Response {
    let text = format!("name: {}", body.name);

    // json形式のメッセージを設定
    let msg = Json(json!({ "message": text}));

    // レスポンス結果を設定して戻り値として返す
    (
        StatusCode::OK,
        msg,
    ).into_response()
}