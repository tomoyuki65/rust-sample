// SeaORM
use sea_orm::{ActiveModelTrait, ColumnTrait, QueryFilter, Set, entity::EntityTrait, TransactionTrait};

// axum
use axum::http::StatusCode;

// chrono
use chrono::{DateTime, FixedOffset, Utc};

// 共通コンテキストの構造体
use crate::api::contexts::context::Context;

// 共通エラー用モジュール
use crate::api::errors::error::CommonError;

// ロガー用のモジュール
use crate::api::loggers::logger::error;

// DB接続用のモジュール
use crate::api::databases::database::db_connection;

// Usersエンティティのモジュール
use crate::api::entities::prelude::{Users, UsersActiveModel, UsersColumn, UsersModel};

// サンプルリポジトリーの構造体
pub struct UsersRepository;

impl UsersRepository {
    // 初期化用メソッド
    pub fn new() -> Self {
        UsersRepository
    }
}

// Usersリポジトリー用のトレイト（モック化もできるように定義）
#[mockall::automock]
#[async_trait::async_trait]
pub trait UsersRepositoryTrait {
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
impl UsersRepositoryTrait for UsersRepository {
    // ユーザー作成
    async fn create_user(
        &self,
        ctx: &Context,
        uid: String,
        last_name: String,
        first_name: String,
        email: String,
    ) -> Result<UsersModel, CommonError> {
        // DB接続
        let db = match db_connection().await {
            Ok(db) => db,
            Err(err) => {
                let msg = format!("[UsersRepository.create_user] DB接続エラー: {}", err);
                error(ctx, &msg);
                return Err(CommonError::CustomError {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: msg,
                });
            }
        };

        // トランザクション開始
        let tx = match db.begin().await {
            Ok(tx) => tx,
            Err(err) => {
                let msg = format!("[UsersRepository.create_user] トランザクション開始エラー: {}", err);
                error(ctx, &msg);
                return Err(CommonError::CustomError {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: msg,
                });
            }
        };

        // ユーザー作成
        let insert_result = Users::insert(UsersActiveModel {
            uid: Set(uid),
            last_name: Set(last_name),
            first_name: Set(first_name),
            email: Set(email),
            ..Default::default()
        })
        .exec(&tx)
        .await;

        match insert_result {
            Ok(insert_result) => {
                // ユーザー登録結果からid取得
                let id = insert_result.last_insert_id;

                // ユーザー情報の取得
                let result = match Users::find_by_id(id).one(&tx).await {
                    Ok(result) => result,
                    Err(err) => {
                        let msg = format!(
                            "[UsersRepository.create_user] ユーザー情報の取得に失敗しました。: {}",
                            err
                        );
                        error(ctx, &msg);
                        return Err(CommonError::CustomError {
                            status_code: StatusCode::INTERNAL_SERVER_ERROR,
                            message: msg,
                        });
                    }
                };
                // 必ずSomeの想定のためunwrap_or_default()を使う。
                let user = result.unwrap_or_default();

                // コミット
                match tx.commit().await {
                    Ok(_) => {}
                    Err(err) => {
                        let msg = format!("[UsersRepository.create_user] コミットエラー: {}", err);
                        error(ctx, &msg);
                        return Err(CommonError::CustomError {
                            status_code: StatusCode::INTERNAL_SERVER_ERROR,
                            message: msg,
                        });
                    }
                }

                return Ok(user);
            }
            Err(err) => {
                let msg = format!(
                    "[UsersRepository.create_user] ユーザー登録に失敗しました。: {}",
                    err
                );
                error(ctx, &msg);
                return Err(CommonError::CustomError {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: msg,
                });
            }
        }
    }

    // 全ての有効なユーザーを取得
    async fn get_users(&self, ctx: &Context) -> Result<Vec<UsersModel>, CommonError> {
        // DB接続
        let db = match db_connection().await {
            Ok(db) => db,
            Err(err) => {
                let msg = format!("[UsersRepository.create_user] DB接続エラー: {}", err);
                error(ctx, &msg);
                return Err(CommonError::CustomError {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: msg,
                });
            }
        };

        // 全ての有効なユーザーを取得
        let select_result = Users::find()
            .filter(UsersColumn::DeletedAt.is_null())
            .all(&db)
            .await;

        // 変換処理
        let users: Vec<UsersModel> = select_result.unwrap_or_default();

        Ok(users)
    }

    // Uidから対象ユーザーを取得
    async fn get_user_from_uid(
        &self,
        ctx: &Context,
        uid: String,
    ) -> Result<Option<UsersModel>, CommonError> {
        // DB接続
        let db = match db_connection().await {
            Ok(db) => db,
            Err(err) => {
                let msg = format!("[UsersRepository.get_user_from_uid] DB接続エラー: {}", err);
                error(ctx, &msg);
                return Err(CommonError::CustomError {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: msg,
                });
            }
        };

        // Uidから有効な対象のユーザーを取得
        let select_result = Users::find()
            .filter(UsersColumn::Uid.eq(uid))
            .filter(UsersColumn::DeletedAt.is_null())
            .one(&db)
            .await;

        let user = match select_result {
            Ok(user) => match user {
                Some(user) => user,
                None => return Ok(None),
            },
            Err(err) => {
                let msg = format!(
                    "[UsersRepository.get_user_from_uid] 対象ユーザー取得エラー: {}",
                    err
                );
                error(ctx, &msg);
                return Err(CommonError::CustomError {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: msg,
                });
            }
        };

        Ok(Some(user))
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
        // DB接続
        let db = match db_connection().await {
            Ok(db) => db,
            Err(err) => {
                let msg = format!("[UsersRepository.update_user] DB接続エラー: {}", err);
                error(ctx, &msg);
                return Err(CommonError::CustomError {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: msg,
                });
            }
        };

        // トランザクション開始
        let tx = match db.begin().await {
            Ok(tx) => tx,
            Err(err) => {
                let msg = format!("[UsersRepository.update_user] トランザクション開始エラー: {}", err);
                error(ctx, &msg);
                return Err(CommonError::CustomError {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: msg,
                });
            }
        };

        // 対象ユーザー取得
        let select_user = Users::find()
            .filter(UsersColumn::Uid.eq(uid))
            .filter(UsersColumn::DeletedAt.is_null())
            .one(&tx)
            .await;

        let mut update_user: UsersActiveModel = match select_user {
            Ok(user) => user.unwrap().into(),
            Err(err) => {
                let msg = format!(
                    "[UsersRepository.update_user] 対象ユーザー取得エラー: {}",
                    err
                );
                error(ctx, &msg);
                return Err(CommonError::CustomError {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: msg,
                });
            }
        };

        // 更新する値の設定
        if !last_name.is_empty() {
            update_user.last_name = Set(last_name);
        }

        if !first_name.is_empty() {
            update_user.first_name = Set(first_name);
        }

        if !email.is_empty() {
            update_user.email = Set(email);
        }

        // 更新日時の設定
        let current_date =
            DateTime::<Utc>::from_naive_utc_and_offset(chrono::Utc::now().naive_utc(), Utc).into();
        update_user.updated_at = Set(current_date);

        // ユーザー更新
        let user: UsersModel = match update_user.update(&tx).await {
            Ok(user) => user,
            Err(err) => {
                let msg = format!(
                    "[UsersRepository.update_user] 対象ユーザー更新エラー: {}",
                    err
                );
                error(ctx, &msg);
                return Err(CommonError::CustomError {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: msg,
                });
            }
        };

        // コミット
        match tx.commit().await {
            Ok(_) => {}
            Err(err) => {
                let msg = format!("[UsersRepository.update_user] コミットエラー: {}", err);
                error(ctx, &msg);
                return Err(CommonError::CustomError {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: msg,
                });
            }
        }

        Ok(user)
    }

    // 対象ユーザー削除（論理削除）
    async fn delete_user(&self, ctx: &Context, uid: String) -> Result<UsersModel, CommonError> {
        // DB接続
        let db = match db_connection().await {
            Ok(db) => db,
            Err(err) => {
                let msg = format!("[UsersRepository.delete_user] DB接続エラー: {}", err);
                error(ctx, &msg);
                return Err(CommonError::CustomError {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: msg,
                });
            }
        };

        // トランザクション開始
        let tx = match db.begin().await {
            Ok(tx) => tx,
            Err(err) => {
                let msg = format!("[UsersRepository.update_user] トランザクション開始エラー: {}", err);
                error(ctx, &msg);
                return Err(CommonError::CustomError {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: msg,
                });
            }
        };

        // 対象ユーザー取得
        let select_user = Users::find()
            .filter(UsersColumn::Uid.eq(uid))
            .filter(UsersColumn::DeletedAt.is_null())
            .one(&tx)
            .await;

        let mut delete_user: UsersActiveModel = match select_user {
            Ok(user) => user.unwrap().into(),
            Err(err) => {
                let msg = format!(
                    "[UsersRepository.delete_user] 対象ユーザー取得エラー: {}",
                    err
                );
                error(ctx, &msg);
                return Err(CommonError::CustomError {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: msg,
                });
            }
        };

        // 現在日時を取得
        let current_date: DateTime<FixedOffset> =
            DateTime::<Utc>::from_naive_utc_and_offset(chrono::Utc::now().naive_utc(), Utc).into();

        // emailをString型に変換し、シングルクォーテーションを削除
        let email_active_value = delete_user.email.into_value().unwrap();
        let email_string: String = email_active_value.to_string();
        let email_trim: String = email_string.trim_matches('\'').to_string();

        // 現在日時をフォーマットしてString型に変換
        let formatted_date: String = current_date.format("%Y%m%d%H%M%S").to_string();

        // emailの設定
        let email: String = format!("{}_{}", email_trim, formatted_date);
        delete_user.email = Set(email);

        // updated_atとdeleted_atの設定
        delete_user.updated_at = Set(current_date);
        delete_user.deleted_at = Set(Some(current_date));

        // ユーザー更新
        let user: UsersModel = match delete_user.update(&tx).await {
            Ok(user) => user,
            Err(err) => {
                let msg = format!(
                    "[UsersRepository.delete_user] 対象ユーザー削除エラー: {}",
                    err
                );
                error(ctx, &msg);
                return Err(CommonError::CustomError {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: msg,
                });
            }
        };

        // コミット
        match tx.commit().await {
            Ok(_) => {}
            Err(err) => {
                let msg = format!("[UsersRepository.delete_user] コミットエラー: {}", err);
                error(ctx, &msg);
                return Err(CommonError::CustomError {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: msg,
                });
            }
        }

        Ok(user)
    }
}
