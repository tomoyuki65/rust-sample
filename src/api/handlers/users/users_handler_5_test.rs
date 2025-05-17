#[cfg(test)]
use test_env_helpers::*;

#[before_each]
#[after_each]
#[cfg(test)]
// get_usersのテスト
mod update_user_test {
    use crate::api::databases::database::db_connection;
    use crate::api::entities::prelude::{Users, UsersActiveModel, UsersColumn, UsersModel};
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
    use serde::{Deserialize, Serialize};

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

    // レスポンス結果の構造体
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Message {
        message: String,
    }

    #[tokio::test]
    async fn it_response_ok() {
        // リクエストを実行
        let url = "http://localhost:8080/api/v1/user/test-xxx-yyy-001";
        let client = reqwest::Client::new();
        let res = client
            .delete(url)
            .header("Authorization", format!("Bearer {}", "xxx"))
            .send()
            .await
            .unwrap();

        // レスポンスステータスの検証
        assert_eq!(res.status(), 200);

        // レスポンスボディの検証
        let text_body = res.text().await.unwrap();
        let req_body: Message = serde_json::from_str(&text_body).unwrap();
        let result_res_body = Message {
            message: "OK".to_string(),
        };
        assert_eq!(req_body, result_res_body);

        // DBからユーザー件数の確認
        let db = match db_connection().await {
            Ok(db) => db,
            Err(err) => {
                panic!("DB接続エラー: {}", err);
            }
        };
        let select_result = Users::find()
            .filter(UsersColumn::DeletedAt.is_null())
            .all(&db)
            .await;
        let users: Vec<UsersModel> = select_result.unwrap_or_default();
        let data_count = users.len();
        assert_eq!(data_count, 1);
    }
}
