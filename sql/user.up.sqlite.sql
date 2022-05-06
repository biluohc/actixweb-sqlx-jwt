-- local: (datetime('now','localtime'))
-- UTC: (datetime('now')
-- https://stackoverflow.com/questions/57052204/utc-time-wrong-in-sqlite
create table users (
    id INTEGER primary key AUTOINCREMENT not null,
    name text UNIQUE not null,
    email char(20) UNIQUE not null,
    pass char(65) not null, -- 'passwd hash'
    create_dt datetime not null default (datetime('now','localtime')), -- 'create datetime'
    update_dt datetime not null default (datetime('now','localtime')), -- 'update datetime'
    status char(10) not null default 'normal' -- comment 'status: normal, blocked, deleted'
);

-- https://www.sqlite.org/quirks.html
-- SQLite has no DATETIME datatype. Instead, dates and times can be stored in any of these ways: 
-- As a TEXT string in the ISO-8601 format. Example: '2018-04-02 12:13:46'.
-- As an INTEGER number of seconds since 1970 (also known as "unix time").
-- As a REAL value that is the fractional Julian day number. 

-- sqlite3 target/lite.db
-- pip3 install litecli
-- litecli target/lite.db

-- .tables
-- create table users
-- .schema users

-- insert INTO users (name, email, pass) values('Alice', 'Alice@google.com', 'passwd');
-- insert INTO users (name, email, pass) values('Bob', 'Bob@google.com', 'passwd');
-- select * from users;
