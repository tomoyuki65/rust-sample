// 共通コンテキストの構造体
use crate::api::contexts::context::Context;

// 共通エラー用モジュール
use crate::api::errors::error::CommonError;

// ロガー用のモジュール
use crate::api::logger::env_logger::error;

// サンプルリポジトリーの構造体
pub struct SampleRepository;

impl SampleRepository {
    // 初期化用メソッド
    pub fn new() -> Self {
        SampleRepository
    }
}

// サンプルリポジトリー用のトレイト（モック化もできるように定義）
#[mockall::automock]
#[async_trait::async_trait]
pub trait SampleRepositoryTrait {
    async fn sample_hello(&self, ctx: &Context) -> Result<String, CommonError>;
}

#[async_trait::async_trait]
impl SampleRepositoryTrait for SampleRepository {
    // 文字列「Sample Hello !!」を返す関数
    async fn sample_hello(&self, ctx: &Context) -> Result<String, CommonError> {
        let text = "Sample Hello !!".to_string();

        if text.is_empty() {
            error(ctx, "textが空です");
            return Err(CommonError::InternalServerError);
        }

        Ok(text)
    }
}
