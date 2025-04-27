// 共通エラー用モジュール
use crate::api::errors::error::CommonError;

// 文字列「Sample Hello !!」を返す関数
pub async fn sample_hello() -> Result<String, CommonError> {
    let text = "Sample Hello !!".to_string();

    if text.is_empty() {
        return Err(CommonError::InternalServerError);
    }

    Ok(text)
}
