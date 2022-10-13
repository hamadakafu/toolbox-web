# toolbox-web
## ENV
`PORT`: appがbindするport
`SERVICE_URL`: schedulerがアクセスするurl
`RUST_LOG`: `env_logger`が参照するlog level
## SECRET ENV
`DATABASE_URL`: appが接続するdbのurl
`BASIC_AUTH_USERNAME`
`BASIC_AUTH_PASSWORD`

## 環境変数の上書き
.envはlocalの環境変数を記述している．
本番環境は別の方法で上書きするようにする．

NOTE: .env < shell指定

## Database
### Dev
sqlxはcompile時に`.env`ファイルの`DATABASE_URL`へアクセスし型checkを行うため，
開発の際にはdocker-composeでpostgresを立ててmigration済みである必要がある．

### Deploy
Deploy時にはdatabaseを前もって作成しておく必要があるが，tableのmigrationは不要．

Dockerのbuild時には環境変数に`SQLX_OFFLINE=true`をセットしておくことでcompileできる．

### fly.io
`flyctl proxy 15432:5432 -a <application_name>`
`pgcli -h localhost -p 15432 -U postgres -W`で接続できる．

toolbox-webとpostgresを接続するには，secretsに`DATABASE_URL`をセットする．
`flyctl postgres attach <postgres_app>`をおこなうとsecretsに`DATABASE_URL`をセットするが，
defaultのuser nameとdatabase nameを自動でセットする．

username: app nameのスネークケース
database: app nameのスネークケース

