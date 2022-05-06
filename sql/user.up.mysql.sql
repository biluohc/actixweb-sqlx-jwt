-- local: datetime(3)
-- UTC: timestamp
create table users (
    id serial primary key,
    name char(10) UNIQUE not null,
    email char(20) UNIQUE not null comment 'email address',
    pass char(65) not null comment 'passwd hash',
    create_dt datetime(3) not null default current_timestamp(3) comment 'create datetime',
    update_dt datetime(3) not null default current_timestamp(3) on update current_timestamp(3) comment 'update datetime',
    status char(10) not null default 'normal' comment 'status: normal, blocked, deleted'
);


-- docker run -d --restart always --name mysql-demo -p 3306:3306 -e MYSQL_ROOT_PASSWORD=MTcwNzUyNzIzMDY4Nzk2MzQ3Mjg= -d mysql:5.7
-- docker exec -it  mysql-demo  /bin/bash
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
