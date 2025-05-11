# RustのaxumによるAPI開発サンプル
Rustのフレームワーク「axum」によるバックエンドAPI開発用サンプルです。  
  
<br />
  
## 要件
・Rustのバージョンは<span style="color:green">1.86</span>です。  
  
<br />
  
## ローカル開発環境構築
### 1. 環境変数ファイルをリネーム
```
cp ./.env.example ./.env
```  
  
### 2. コンテナのビルドと起動
```
docker compose build --no-cache
docker compose up -d
```  
> <span style="color:red">※テストコードを実行させる際はテスト用の環境変数ファイルを使うため、「docker compose --env-file ./.env.testing up -d」で起動すること。</span>
  
### 3. コンテナの停止・削除
```
docker compose down
```  
  
<br />
  
## コード修正後に使うコマンド
ローカルサーバー起動中に以下のコマンドを実行可能です。  
  
### 1. フォーマット修正
```
docker compose exec api cargo fmt
```  
  
### 2. コード解析チェック
```
docker compose exec api cargo clippy
```  
  
<br />
  
## OpenAPIのファイル出力用コマンド
ローカルサーバー起動後に以下のコマンドを実行し、OpenAPI仕様書をディレクトリ「src/api/openapi」にJSON形式で出力可能です。
```
docker compose exec api rust-script ./src/script_openapi.rs
```  
  
<br />
  
## 本番環境用のコンテナについて
本番環境用コンテナをローカルでビルドして確認したい場合は、以下の手順で行って下さい。  
  
### 1. コンテナのビルド
以下のコマンドを実行し、コンテナをビルドします。  
```
docker build --no-cache -f ./docker/prod/Dockerfile -t rust-sample:latest .
```  
  
### 2. コンテナの起動
以下のコマンドを実行し、コンテナを起動します。  
```
docker run -d -p 80:8080 --env-file .env.production rust-sample:latest
```  
  