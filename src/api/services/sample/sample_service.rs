// 共通コンテキストの構造体
use crate::api::contexts::context::Context;

// 共通エラー用モジュール
use crate::api::errors::error::CommonError;

// リポジトリ用のモジュール
use crate::api::repositories::sample::sample_repository::SampleRepositoryTrait;

// ロガー用のモジュール
use crate::api::logger::env_logger::error;

// 使用するリポジトリーをまとめる構造体
pub struct SampleCommonRepository {
    // Box<T>型で動的にメモリ領域確保
    // Send: オブジェクトが異なるスレッド間で安全に送信できることを保証
    // Sync: オブジェクトが複数のスレッドから同時にアクセスできることを保証
    // 'static: オブジェクトのライフタイムがプログラムが終了するまで破棄されない
    pub sample_repo: Box<dyn SampleRepositoryTrait + Send + Sync + 'static>,
}

// サンプルサービス
pub struct SampleService {
    repo: SampleCommonRepository,
}

impl SampleService {
    pub fn new(repo: SampleCommonRepository) -> Self {
        SampleService { repo }
    }
}

// サンプルサービス用のトレイト（モック化もできるように定義）
#[mockall::automock]
#[async_trait::async_trait]
pub trait SampleServiceTrait {
    async fn sample_get_text_hello(&self, ctx: &Context) -> Result<String, CommonError>;
}

#[async_trait::async_trait]
impl SampleServiceTrait for SampleService {
    async fn sample_get_text_hello(&self, ctx: &Context) -> Result<String, CommonError> {
        let text = match self.repo.sample_repo.sample_hello(ctx).await {
            Ok(text) => text,
            Err(err) => {
                error(ctx, "sample_get_text_helloのsample_hello処理でエラー");
                return Err(err);
            }
        };

        Ok(text)
    }
}
