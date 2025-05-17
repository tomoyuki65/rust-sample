use envy;
use serde::Deserialize;

// 環境変数のデフォルト値を返す関数
fn default_env() -> String {
    "local".to_string()
}

fn default_port() -> u16 {
    8080
}

fn default_rust_log() -> String {
    "info".to_string()
}

fn default_database_url() -> String {
    "postgres://pg-user:pg-password@pg-db:5432/pg-db?sslmode=disable".to_string()
}

// 環境変数の構造体
#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_env")]
    pub env: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[allow(dead_code)]
    #[serde(default = "default_rust_log")]
    pub rust_log: String,
    #[serde(default = "default_database_url")]
    pub database_url: String,
}

// 環境変数を返す関数
pub fn get_config() -> Config {
    match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(err) => {
            println!("環境変数の初期化エラー: {}", err);

            // 環境変数にデフォルト値を設定して返す
            Config {
                env: default_env(),
                port: default_port(),
                rust_log: default_rust_log(),
                database_url: default_database_url(),
            }
        }
    }
}
