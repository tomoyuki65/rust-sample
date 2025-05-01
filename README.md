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
  