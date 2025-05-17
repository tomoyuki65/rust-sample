use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

// configsモジュール
use crate::api::configs::config;

// DB接続
pub async fn db_connection() -> Result<DatabaseConnection, DbErr> {
    // 環境変数取得
    let config = config::get_config();

    // DB接続
    let mut opt = ConnectOptions::new(config.database_url);
    // sqlxのログ出力をOFFに変更
    opt.sqlx_logging(false);
    let db = Database::connect(opt).await?;

    Ok(db)
}
