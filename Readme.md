# RoleMater
Discordを利用したイベント抽選bot

## 必須ツール
* cargo
* sqlx-cli
* (Ubuntuの場合) build-essential libffi-dev pkg-config libssl-dev

## DBマイグレーション
### ツールのインストール
```sh
cargo install sqlx-cli
```

### 環境変数の設定
sqliteの場合
```powershell
$Env:DATABASE_URL = "sqlite:./database.db"
```
or
```sh
DATABASE_URL = "sqlite:./database.db"
```
mysqlの場合
```powershell
$Env:DATABASE_URL = "mysql:password@localhost:3306/my_database"
```
or
```sh
DATABASE_URL = "mysql:password@localhost:3306/my_database"
```
### DBの生成
```sh
sqlx database create
```

### マイグレーションファイルの作成
```sh
sqlx migrate add -r <name>
```

### マイグレーションの実行
```sh
sqlx migrate run
```

### リバートの実行
```sh
sqlx migrate revert
```

## デバッグ時の注意点
Dockerで動かしている場合、コードを編集しても再ビルドが走らないので明示的に再ビルドを行う必要があります。