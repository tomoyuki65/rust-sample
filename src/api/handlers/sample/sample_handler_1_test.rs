#[cfg(test)]

// sample_getのテスト
mod sample_get_test {
    use crate::api::contexts::context::Context;
    use crate::api::repositories::sample::sample_repository::SampleRepository;
    use crate::api::services::sample::sample_service::{SampleCommonRepository, SampleService};
    use crate::api::usecases::sample::sample_get_usecase::{SampleCommonService, SampleGetUsecase};
    use axum::body;
    use axum::http::header::HeaderMap;
    use serde::{Deserialize, Serialize};

    // レスポンス結果の構造体
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Message {
        message: String,
    }

    #[tokio::test]
    async fn it_response_ok() {
        /* ユースケースを実行して検証する場合 */

        // サンプルリポシトリーのインスタンス化
        let sample_repo = Box::new(SampleRepository::new());

        /*
            リポジトリーのモック化が必要な場合の例
            let mut mock_repo = MockSampleRepositoryTrait::new();
            mock_repo
                .expect_sample_hello()
                .returning(|_| Ok("mock".to_string()));
            let sample_repo = Box::new(mock_repo);
        */

        // サンプルサービスのインスタンス化
        let sample_common_repo = SampleCommonRepository { sample_repo };
        let sample_service = SampleService::new(sample_common_repo);
        let sample_common_service = SampleCommonService { sample_service };

        // ユースケースを実行
        let sample_get_usecase = SampleGetUsecase {
            service: sample_common_service,
        };
        let ctx = Context {
            header: HeaderMap::new(),
            method: "GET".to_string(),
            uri: "/api/v1/sample/get".to_string(),
        };
        let res = sample_get_usecase.exec(ctx).await;

        // レスポンスステータスの検証
        assert_eq!(res.status(), 200);

        // レスポンスボディの検証
        let limit = 1024 * 1024;
        let body_bytes = body::to_bytes(res.into_body(), limit).await.unwrap();
        let body_str = String::from_utf8_lossy(&body_bytes);
        let req_body: Message = serde_json::from_str(&body_str).unwrap();
        let result_res_body = Message {
            message: "Sample Hello !!".to_string(),
        };
        assert_eq!(req_body, result_res_body);
    }
}

// sample_get_path_queryのテスト
mod sample_get_path_query_test {
    use serde::{Deserialize, Serialize};

    // レスポンス結果の構造体
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Message {
        message: String,
    }

    #[tokio::test]
    async fn it_response_ok() {
        /* reqwestを使ってHTTPリクエストによる検証をする場合 */

        // リクエストを実行
        let url = "http://localhost:8080/api/v1/sample/get/11?item=book";
        let client = reqwest::Client::new();
        let res = client.get(url).send().await.unwrap();

        // レスポンスステータスの検証
        assert_eq!(res.status(), 200);

        // レスポンスボディの検証
        let text_body = res.text().await.unwrap();
        let req_body: Message = serde_json::from_str(&text_body).unwrap();
        let result_res_body = Message {
            message: "id: 11, item: book".to_string(),
        };
        assert_eq!(req_body, result_res_body);
    }
}

// sample_postのテスト
mod sample_post_test {
    use serde::{Deserialize, Serialize};

    // レスポンス結果の構造体
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Message {
        message: String,
    }

    #[tokio::test]
    async fn it_response_ok() {
        // リクエストを実行
        let url = "http://localhost:8080/api/v1/sample/post";
        let data = serde_json::json!({
            "name": "田中"
        });
        let client = reqwest::Client::new();
        let res = client.post(url).json(&data).send().await.unwrap();

        // レスポンスステータスの検証
        assert_eq!(res.status(), 200);

        // レスポンスボディの検証
        let text_body = res.text().await.unwrap();
        let req_body: Message = serde_json::from_str(&text_body).unwrap();
        let result_res_body = Message {
            message: "name: 田中".to_string(),
        };
        assert_eq!(req_body, result_res_body);
    }
}
