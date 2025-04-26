// 共通エラー用モジュール
use crate::api::errors::error::CommonError;

// リポジトリ用のモジュール
use crate::api::repositories::sample::sample_repository;

// サンプルテキストを取得するサービス
pub async fn sample_get_text_hello() -> Result<String, CommonError> {
    let text = match sample_repository::sample_hello().await {
        Ok(text) => text,
        Err(err) => return Err(err),
    };

    Ok(text)
}
