// 共通コンテキストの構造体
use crate::api::contexts::context::Context;

// 共通エラー用モジュール
use crate::api::errors::error::CommonError;

// リポジトリ用のモジュール
use crate::api::repositories::users::users_repository::UsersRepositoryTrait;

// Usersモデル
use crate::api::entities::prelude::UsersModel;

// 使用するリポジトリーをまとめる構造体
pub struct UsersCommonRepository {
    // Box<T>型で動的にメモリ領域確保
    // Send: オブジェクトが異なるスレッド間で安全に送信できることを保証
    // Sync: オブジェクトが複数のスレッドから同時にアクセスできることを保証
    // 'static: オブジェクトのライフタイムがプログラムが終了するまで破棄されない
    pub users_repo: Box<dyn UsersRepositoryTrait + Send + Sync + 'static>,
}

// サンプルサービス
pub struct UsersService {
    repo: UsersCommonRepository,
}

impl UsersService {
    pub fn new(repo: UsersCommonRepository) -> Self {
        UsersService { repo }
    }
}

// Usersサービス用のトレイト（モック化もできるように定義）
#[mockall::automock]
#[async_trait::async_trait]
pub trait UsersServiceTrait {
    async fn create_user(
        &self,
        ctx: &Context,
        uid: String,
        last_name: String,
        first_name: String,
        email: String,
    ) -> Result<UsersModel, CommonError>;
    async fn get_users(&self, ctx: &Context) -> Result<Vec<UsersModel>, CommonError>;
    async fn get_user_from_uid(
        &self,
        ctx: &Context,
        uid: String,
    ) -> Result<Option<UsersModel>, CommonError>;
    async fn update_user(
        &self,
        ctx: &Context,
        uid: String,
        last_name: String,
        first_name: String,
        email: String,
    ) -> Result<UsersModel, CommonError>;
    async fn delete_user(&self, ctx: &Context, uid: String) -> Result<UsersModel, CommonError>;
}

#[async_trait::async_trait]
impl UsersServiceTrait for UsersService {
    // ユーザー作成
    async fn create_user(
        &self,
        ctx: &Context,
        uid: String,
        last_name: String,
        first_name: String,
        email: String,
    ) -> Result<UsersModel, CommonError> {
        // ユーザー作成処理
        let user = match self
            .repo
            .users_repo
            .create_user(ctx, uid, last_name, first_name, email)
            .await
        {
            Ok(user) => user,
            Err(err) => {
                return Err(err);
            }
        };

        Ok(user)
    }

    // 全ての有効なユーザー取得
    async fn get_users(&self, ctx: &Context) -> Result<Vec<UsersModel>, CommonError> {
        let users = match self.repo.users_repo.get_users(ctx).await {
            Ok(users) => users,
            Err(err) => {
                return Err(err);
            }
        };

        Ok(users)
    }

    // Uidから有効な対象ユーザー取得
    async fn get_user_from_uid(
        &self,
        ctx: &Context,
        uid: String,
    ) -> Result<Option<UsersModel>, CommonError> {
        let user = match self.repo.users_repo.get_user_from_uid(ctx, uid).await {
            Ok(user) => user,
            Err(err) => {
                return Err(err);
            }
        };

        Ok(user)
    }

    // 対象ユーザー更新
    async fn update_user(
        &self,
        ctx: &Context,
        uid: String,
        last_name: String,
        first_name: String,
        email: String,
    ) -> Result<UsersModel, CommonError> {
        let user = match self
            .repo
            .users_repo
            .update_user(ctx, uid, last_name, first_name, email)
            .await
        {
            Ok(user) => user,
            Err(err) => {
                return Err(err);
            }
        };

        Ok(user)
    }

    // 対象ユーザー削除
    async fn delete_user(&self, ctx: &Context, uid: String) -> Result<UsersModel, CommonError> {
        let user = match self.repo.users_repo.delete_user(ctx, uid).await {
            Ok(user) => user,
            Err(err) => {
                return Err(err);
            }
        };

        Ok(user)
    }
}
