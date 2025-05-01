// axum
use axum::{extract::Request, http::header::HeaderMap};

// 共通コンテキストの構造体
#[derive(Clone)]
pub struct Context {
    pub header: HeaderMap,
    pub method: String,
    pub uri: String,
}

// コンテキスト作成関数
pub fn create_context(req: &Request) -> Context {
    let mut hm = HeaderMap::new();
    for (key, value) in req.headers().iter() {
        hm.insert(key.clone(), value.clone());
    }

    Context {
        header: hm,
        method: req.method().to_string(),
        uri: req.uri().to_string(),
    }
}
