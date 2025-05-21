# RustのaxumによるAPI開発サンプル
Rustのフレームワーク「axum」によるバックエンドAPI開発用サンプルです。  
  
<br />
  
## 要件
・Rustのバージョンは<span style="color:green">1.87</span>です。  
  
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
  
### 3. テストコードの実行
<span style="color:red">事前にテスト用環境変数を設定したローカルサーバーを起動（docker compose --env-file ./.env.testing up -d）してから以下のコマンドを使ってテストを実行して下さい</span>  
```
docker compose exec -e CARGO_TEST=testing api cargo test -- --nocapture --test-threads=1
```  
> ※DBのデータを同期させるため「--test-threads=1」で実行する
  
<br />
  
## マイグレーションに関する操作用コマンド
ローカルサーバー起動中に以下のコマンドを実行可能です。  
  
### 1. マイグレーションの状態確認
```
docker compose exec api sea-orm-cli migrate status
```  
  
### 2. マイグレーションの実行
```
docker compose exec api sea-orm-cli migrate up
```  
  
### 3. ロールバックの実行
```
docker compose exec api sea-orm-cli migrate down
```  
  
### 4. マイグレーションファイルの新規作成
```
docker compose exec api sea-orm-cli migrate generate ファイル名
```  
> ※ファイル名の例「create_table_xxxx」
  
### 5. entitiesの追加
マイグレーション完了後に以下のコマンドを実行し、entitiesの追加が可能です。  
```
docker compose exec api sea-orm-cli generate entity -o src/api/entities/tmp --tables xxxx
```  
> <span style="color:red">※テーブル名を指定してコマンドを実行し、tmpディレクトリに作成されたファイル内容を手動で追加後、最後にtmpディレクトリを削除して下さい。（entitiesの上書きを防ぐため）</span>
  
<br />
  
## OpenAPIのファイル出力用コマンド
ローカルサーバー起動後に以下のコマンドを実行し、OpenAPI仕様書をディレクトリ「src/api/openapi」にJSON形式で出力可能です。
```
docker compose exec api rust-script ./src/script_openapi.rs
```  
  
<br />
  
## 本番環境用のコンテナについて
本番環境用コンテナをローカルでビルドして確認したい場合は、以下の手順で行って下さい。  
  
### 1. .env.productionの修正
本番環境用の機密情報を含まない環境変数の設定には「.env.production」を使っていますが、ローカルで確認したい場合はローカル用と同様に機密情報も含む環境変数も追加して下さい。
```
DATABASE_URL=postgres://pg-user:pg-password@host.docker.internal:5432/pg-db
```
  
### 2. コンテナのビルド
以下のコマンドを実行し、コンテナをビルドします。  
```
docker build --no-cache -f ./docker/prod/Dockerfile -t rust-sample:latest .
```  
  
### 3. コンテナの起動
以下のコマンドを実行し、コンテナを起動します。  
```
docker run -d -p 80:8080 --env-file .env.production rust-sample:latest
```  
  