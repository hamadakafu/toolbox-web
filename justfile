deploy:
  git push heroku master

sqlx-database-create:
  sqlx database create

sqlx-database-drop:
  sqlx database drop

sqlx-migrate-add NAME:
  sqlx migrate add {{NAME}}

sqlx-migrate-run:
  sqlx migrate run

pg:
  pgcli -h localhost -p 5432 -U postgres -W -d toolbox

push-text TEXT="hoge":
  curl -v -u username:password -X POST -H 'Content-type: application/json' -d '{"value": "{{TEXT}}"}' localhost:8080/texts

pop-text-id UUID="0f655cc5a73e4a3188f445b44f5cf4cc":
  curl -v -u username:password localhost:8080/texts/{{UUID}}

pop-text:
  curl -v -u username:password localhost:8080/texts/latest

delete-text UUID="6feec6efe26b41b5bbedb8b2731ebc47":
  curl -v -u username:password -X DELETE localhost:8080/texts/{{UUID}}

delete-text-old:
  curl -v -u username:password -X DELETE localhost:8080/texts/old

# CIでquery!マクロが静的に解決出来るようにsqlx-data.jsonを作る．
sqlx-prepare:
  cargo sqlx prepare
