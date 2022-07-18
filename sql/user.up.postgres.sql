-- local: timestamp
-- UTC: timestamptz

create table user_address2 (
        address varchar(65) not null,
        experience varchar(10) UNIQUE not null
);

-- COMMENT ON COLUMN "users"."pass" IS 'passwd hash'

-- docker run -d --restart always --name pg-demo -e POSTGRES_USER=template -e POSTGRES_DB=templatedb  -e POSTGRES_PASSWORD=MTcwNzUyNzIzMDY4Nzk2MzQ3Mjg= -p 5432:5432  postgres:14

-- libpg-dev/postgresql-devel
-- pip3 install pgcli
-- pgcli postgres://template:MTcwNzUyNzIzMDY4Nzk2MzQ3Mjg=@localhost:5432/templatedb
-- create table users
-- \db; \l+; \di; \d users;

-- DATABASE_URL="postgres://template:MTcwNzUyNzIzMDY4Nzk2MzQ3Mjg=@localhost:5432/templatedb"
