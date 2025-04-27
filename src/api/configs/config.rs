use serde::{Deserialize};
use envy;

// 環境変数のデフォルト値を返す関数
fn default_env() -> String {
    "local".to_string()
}

fn default_port() -> u16 {
    8080
}

// 環境変数の構造体
#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default="default_env")]
    pub env: String,
    #[serde(default="default_port")]
    pub port: u16,
}

// 環境変数を返す関数
pub fn get_config() -> Config {

    match envy::from_env::<Config>() {
        Ok(config) => {
            config
        },
        Err(err) => {
            println!("環境変数の初期化エラー: {}", err);
    
            // 環境変数にデフォルト値を設定して返す
            Config {env: default_env(), port: default_port()}
        }
    }
}