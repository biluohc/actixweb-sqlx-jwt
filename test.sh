#!/bin/bash

clean() {
    git checkout -f .env Cargo.toml template.json
}

bd=`cargo metadata --format-version 1|jq -r .target_directory`
test() {
    rm -rf $bd/release/actixweb-sqlx-jwt

    cargo build --release
    echo $bd/release/actixweb-sqlx-jwt -v|sh &
    sleep 5

    curl 0.0.0.0:8080/static || echo "Test for $1 failed"

    pgrep -f actixweb-sqlx-jwt |xargs kill -9 &
    rm -frv $bd/release/actixweb-sqlx-jwt
    sleep 10
}

test_and_clean() {
    clean

    kind=$1
    echo "Test for $kind prepare"

    sed -i "s/default\ =\ \[\ \"mysql\"\ \]/default = [ \"$kind\" ]/g" Cargo.toml 
    test $kind = 'mysql' || (cat .env |grep $kind|sed  's/\#//gw .env')

    json=`cat template.json|grep -v sql`
    db=`cat template.json|grep $kind|sed 's/\/\/ //g'`
    echo $json |sed "s#{#{$db#g" |jq . > template.json

    echo "Test for $kind .."
    test $kind
    echo "Test for $kind Ok: $?\n"

    clean
}


dbs=(mysql postgres sqlite)
for db in ${dbs[@]}
do
test_and_clean $db 
done
