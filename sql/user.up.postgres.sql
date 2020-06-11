-- local: timestamp
create table users (
    id serial primary key,
    name varchar(10) UNIQUE not null,
    email varchar(20) UNIQUE not null,
    pass varchar(65) not null, -- 'passwd hash'
    create_dt timestamptz not null default current_timestamp, -- '创建时间'
    update_dt timestamptz not null default current_timestamp -- '更新时间'
    -- status char(10) not null default 'nomal' -- '状态, 正常: normal, 封禁: blocked',
);

-- COMMENT ON COLUMN "users"."pass" IS 'passwd hash'

-- docker run -it -d --name postgresql -e POSTGRES_USER=template -e POSTGRES_DB=templatedb  -e POSTGRES_PASSWORD=MTcwNzUyNzIzMDY4Nzk2MzQ3Mjg= -p 5432:5432  postgres:12

-- pip3 install pgcli
-- pgcli postgres://template:MTcwNzUyNzIzMDY4Nzk2MzQ3Mjg=@localhost:5432/templatedb
-- create table users
-- \db; \l+; \di; \d users;

-- DATABASE_URL="postgres://template:MTcwNzUyNzIzMDY4Nzk2MzQ3Mjg=@localhost:5432/templatedb"
