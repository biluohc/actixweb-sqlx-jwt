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
curl -v --data '{"name": "Bob", "email": "Bob@google.com", "password": "Bobpass"}' -H "Content-Type: application/json" -X POST localhost:8080/api/user/register

curl -v --data '{"name": "Bob", "email": "Bob@google.com", "password": "Bobpass"}' -H "Content-Type: application/json" -X POST localhost:8080/api/user/login

curl -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJCb2IiLCJleHAiOjE1OTEyNDYwOTR9.O1dbYu3tqiIi6I8OUlixLuj9dp-1tLl4mjmXZ0ve6uo' localhost:8080/api/user/info

curl 'localhost:8080/api/user/info?access_token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJCb2IiLCJleHAiOjE1OTEyNTYxNDd9.zJKlZOozYfq-xMXO89kjUyme6SA8_eziacqt5gvXj2U'
```
6. Modify the code and write your own code, enjoy.

## Notice
1. The sqlx query macros needs to be connected to the database represented by DATABASE_URL in .env, or you can consider using the unchecked version instead.

## Redis: No Redis does not affect the program running now
1. Use docker to start a Redis quickly
```bash
docker run --name redis-6379 --network host -d redis redis-server --port 6379 --bind 127.0.0.1 --appendonly  no  # --requirepass pw

# pip3 install iredis

# iredis/redis-cli -p 6379 # -a pw
```
2. The Redis client crates, current is mobc.
    1. [redis-rs](https://github.com/mitsuhiko/redis-rs): The most used Redis client.
    1. [mobc](https://github.com/importcjj/mobc): An asynchronous connection pool.
    1. [deadpool](https://github.com/bikeshedder/deadpool): An asynchronous connection pool.
    1. [actix-redis](https://github.com/actix/actix-extras/tree/master/actix-redis): Redis integration for actix framework base on redis-async-rs.
    1. [bb8](https://crates.io/crates/bb8): An asynchronous connection pool provides the same configuration options as r2d2.
    2. [r2d2](https://github.com/sfackler/r2d2): A synchronized connection pool, not recommended.
    3. [redis-async-rs](https://github.com/benashford/redis-async-rs): Another Redis client.

### References
1. salvo: https://github.com/salvo-rs/salvo
1. actix-web: https://github.com/actix/actix-web
2. sqlx: https://github.com/launchbadge/sqlx
2. actix documentation: https://actix.rs/docs/
2. actix-web-jwt with mongodb: https://github.com/emreyalvac/actix-web-jwt
2. actix-examples: https://github.com/actix/examples
2. an instance: https://github.com/biluohc/KeepStats
