#[cfg(test)]
use test_env_helpers::*;

#[before_each]
#[after_each]
#[cfg(test)]
// get_usersのテスト
mod get_users_test {
    use crate::api::databases::database::db_connection;
    use crate::api::entities::prelude::{Users, UsersActiveModel, UsersModel};
    use sea_orm::{EntityTrait, Set};

    // テスト前に実行する処理
    async fn before_each() {
        // DB接続
        let db = match db_connection().await {
            Ok(db) => db,
            Err(err) => {
                panic!("DB接続エラー: {}", err);
            }
        };

        // ユーザー作成
        Users::insert(UsersActiveModel {
            uid: Set("test-xxx-yyy-001".to_string()),
            last_name: Set("田中".to_string()),
            first_name: Set("太郎".to_string()),
            email: Set("t.tanaka@example.com".to_string()),
            ..Default::default()
        })
        .exec(&db)
        .await
        .unwrap();

        Users::insert(UsersActiveModel {
            uid: Set("test-xxx-yyy-002".to_string()),
            last_name: Set("田中".to_string()),
            first_name: Set("次郎".to_string()),
            email: Set("ziro.tanaka@example.com".to_string()),
            ..Default::default()
        })
        .exec(&db)
        .await
        .unwrap();
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
        let url = "http://localhost:8080/api/v1/users";
        let client = reqwest::Client::new();
        let res = client
            .get(url)
            .header("Authorization", format!("Bearer {}", "xxx"))
            .send()
            .await
            .unwrap();

        // レスポンスステータスの検証
        assert_eq!(res.status(), 200);

        // レスポンスボディの検証
        let text_body = res.text().await.unwrap();
        let req_body: Vec<UsersModel> = serde_json::from_str(&text_body).unwrap();
        let data_len = req_body.len();
        assert_eq!(data_len, 2);
        assert_eq!(req_body[0].uid, "test-xxx-yyy-001");
        assert_eq!(req_body[0].last_name, "田中");
        assert_eq!(req_body[0].first_name, "太郎");
        assert_eq!(req_body[0].email, "t.tanaka@example.com");
        assert_eq!(req_body[1].uid, "test-xxx-yyy-002");
        assert_eq!(req_body[1].last_name, "田中");
        assert_eq!(req_body[1].first_name, "次郎");
        assert_eq!(req_body[1].email, "ziro.tanaka@example.com");
    }
}
