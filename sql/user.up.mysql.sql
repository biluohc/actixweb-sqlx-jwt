create table users (
    id serial primary key,
    name char(10) UNIQUE not null,
    email char(20) UNIQUE not null comment 'email address',
    pass char(65) not null comment 'passwd hash',
    create_dt timestamp not null default current_timestamp comment '创建时间',
    update_dt timestamp not null default current_timestamp comment '更新时间'
    -- status char(10) not null default 'nomal' comment '状态, 正常: normal, 封禁: blocked',
);


-- docker container run --name mysql-demo -p 3306:3306 -e MYSQL_ROOT_PASSWORD=MTcwNzUyNzIzMDY4Nzk2MzQ3Mjg= -d mysql:5.7
-- docker container exec -it  mysql-demo  /bin/bash
-- mysql -u root -pMTcwNzUyNzIzMDY4Nzk2MzQ3Mjg=
-- grant all privileges on *.* to 'root'@'%' identified by '[password]';
-- select host, user, authentication_string from user;
-- update user set authentication_string=PASSWORD("MTcwNzUyNzIzMDY4Nzk2MzQ3Mjg=") where user="root";
-- flush privileges;

-- docker restart mysql-demo 

-- create database templatedb;
-- use templatedb;
-- show databases;

-- create table users
-- SHOW CREATE TABLE users;
-- show tables; desc users;

-- insert INTO users (name, email, pass) values('Alice', 'Alice@google.com', 'passwd');
-- insert INTO users (name, email, pass) values('Bob', 'Bob@google.com', 'passwd');
-- select * from users;

-- pip3 install mycli
-- mycli -hlocalhost -uroot -pMTcwNzUyNzIzMDY4Nzk2MzQ3Mjg= -Dtemplatedb
-- DATABASE_URL=mysql://root:MTcwNzUyNzIzMDY4Nzk2MzQ3Mjg=@localhost/templatedb?timeout=60&serverTimezone=Hongkong
