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
curl -v --data '{"name": "Bob", "email": "Bob@google.com", "password": "Bobpass"}' -H "Content-Type: application/json" -X POST localhost:8080/user/login
curl -v --data '{"name": "Bob", "email": "Bob@google.com", "password": "Bobpass"}' -H "Content-Type: application/json" -X POST localhost:8080/user/register
```
6. Modify the code and write your own code, enjoy.

## Notice
1. The sqlx query macros needs to be connected to the database represented by DATABASE_URL in .env, or you can consider using the unchecked version instead.
