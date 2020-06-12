# actixweb-sqlx-jwt
    A jwt template project of actix-web and sqlx

## sqlx-cli
cargo install sqlx-cli --git https://github.com/launchbadge/sqlx

## Usage

1. Choose a database(`mysql`, `postgres`, `sqlite`).
2. Sets the default feature as the database name on Cargo.toml(current is `mysql`).
3. Configure the databse you can see `sql/user.up.$database.sql`.
4. Run `cargo run -- -v` after update .env and template.json.
5. Test current api:
```sh
curl -v --data '{"name": "Bob", "email": "Bob@google.com", "password": "Bobpass"}' -H "Content-Type: application/json" -X POST localhost:8080/user/register

curl -v --data '{"name": "Bob", "email": "Bob@google.com", "password": "Bobpass"}' -H "Content-Type: application/json" -X POST localhost:8080/user/login

curl -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJCb2IiLCJleHAiOjE1OTEyNDYwOTR9.O1dbYu3tqiIi6I8OUlixLuj9dp-1tLl4mjmXZ0ve6uo' localhost:8080/user/userInfo

curl 'localhost:8080/user/userInfo?access_token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJCb2IiLCJleHAiOjE1OTEyNTYxNDd9.zJKlZOozYfq-xMXO89kjUyme6SA8_eziacqt5gvXj2U'
```
6. Modify the code and write your own code, enjoy.

## Notice
1. The sqlx query macros needs to be connected to the database represented by DATABASE_URL in .env, or you can consider using the unchecked version instead.

### References
1. actix-web: https://github.com/actix/actix-web
2. sqlx: https://github.com/launchbadge/sqlx
2. actix documentation: https://actix.rs/docs/
2. actix-web-jwt with mongodb: https://github.com/emreyalvac/actix-web-jwt
2. actix-examples: https://github.com/actix/examples
