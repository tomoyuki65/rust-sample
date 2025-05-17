#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! async-trait = "0.1.88"
//! axum = "0.8.3"
//! chrono = "0.4.40"
//! env_logger = "0.11.8"
//! envy = "0.4.2"
//! log = "0.4.27"
//! mockall = "0.13.1"
//! reqwest = { version = "0.12.15", features = ["json"] }
//! sea-orm = { version = "1.1.11", features = [ "sqlx-postgres", "runtime-tokio-rustls", "macros" ] }
//! serde = { version = "1.0.219", features = ["derive"] }
//! serde_json = "1.0.140"
//! thiserror = "2.0.12"
//! tokio = { version = "1.44.2", features = ["full"] }
//! tower-http = { version = "0.6.2", features = ["trace"] }
//! tracing = "0.1.41"
//! utoipa = { version = "5.3.1", features = ["axum_extras"] }
//! utoipa-swagger-ui = { version = "9.0.0", features = ["axum"] }
//! uuid = { version = "1.16.0", features = ["v4"] }
//! validator = { version = "0.20.0", features = ["derive"] }
// //! ```

// OpenAPI用
use utoipa::OpenApi;

// ファイル出力用
use std::fs::File;
use std::io::Write;
use std::path::Path;
use serde_json;

// apiモジュール
mod api;

// routerモジュール
use api::router::ApiDoc;

fn main() {
    // OpenAPIをファイルに出力
    let output_path = Path::new("./src/api/openapi/openapi.json");
    let json_string = serde_json::to_string_pretty(&ApiDoc::openapi()).unwrap();
    let file = File::create(output_path);
    let _ = file.unwrap().write_all(json_string.as_bytes());
}
