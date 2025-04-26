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
  
### 3. コンテナの停止・削除
```
docker compose down
```  
  
<br />
  