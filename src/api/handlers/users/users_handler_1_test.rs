#[cfg(test)]
use test_env_helpers::*;

#[before_each]
#[after_each]
#[cfg(test)]
// create_userのテスト
mod create_user_test {
    use crate::api::databases::database::db_connection;
    use crate::api::entities::prelude::{Users, UsersModel};
    use sea_orm::EntityTrait;

    // テスト前に実行する処理
    async fn before_each() {
        // DB接続
        let db = match db_connection().await {
            Ok(db) => db,
            Err(err) => {
                panic!("DB接続エラー: {}", err);
            }
        };

        // usersテーブルのデータを全て削除
        Users::delete_many().exec(&db).await.unwrap();
    }

    // テスト後に実行する処理
    async fn after_each() {
        // DB接続
        let db = match db_connection().await {
            Ok(db) => db,
            Err(err) => {
                panic!("DB接続エラー: {}", err);
            }
        };

        // usersテーブルのデータを全て削除
        Users::delete_many().exec(&db).await.unwrap();
    }

    #[tokio::test]
    async fn it_response_ok() {
        // リクエストを実行
        let url = "http://localhost:8080/api/v1/user";
        let data = serde_json::json!({
            "last_name": "田中",
            "first_name": "太郎",
            "email": "t.tanaka@example.com"
        });
        let client = reqwest::Client::new();
        let res = client.post(url).json(&data).send().await.unwrap();

        // レスポンスステータスの検証
        assert_eq!(res.status(), 201);

        // レスポンスボディの検証
        let text_body = res.text().await.unwrap();
        let req_body: UsersModel = serde_json::from_str(&text_body).unwrap();
        assert_eq!(req_body.last_name, "田中");
        assert_eq!(req_body.first_name, "太郎");
        assert_eq!(req_body.email, "t.tanaka@example.com");
    }
}
