#!/bin/bash
# https://docs.julialang.org/en/v1.2/manual/faq/#How-do-I-catch-CTRL-C-in-a-script?-1
#= 
exec julia --color=yes -e 'include(popfirst!(ARGS))' \
    "${BASH_SOURCE[0]}" "$@" =#


using Test

checkout() = run(`git checkout -f .env Cargo.toml template.json`)

bd = read(pipeline(`cargo metadata --format-version 1`, `jq -r .target_directory`), String) |> strip
clean() = run(`rm -frv $bd/release/actixweb-sqlx-jwt`)

function test(kind::String)
    clean()

    run(`cargo build --release`)
    proc = run(`$bd/release/actixweb-sqlx-jwt -v`, wait=false)
    # wait server startup
    sleep(1)

    try
        run(`curl 0.0.0.0:8080/assets`)
        run(`curl -s --data '{"name": "Bob", "email": "Bob@google.com", "password": "Bobpass"}' -H "Content-Type: application/json" -X POST localhost:8080/api/user/register`)

        jwt = read(pipeline(`curl -s --data '{"name": "Bob", "email": "Bob@google.com", "password": "Bobpass"}' -H "Content-Type: application/json" -X POST localhost:8080/api/user/login`, `jq -r .data`), String) |> strip
        @test length(jwt) > 100

        authead = "Authorization: Bearer $jwt"
        code = read(pipeline(`curl -sH $authead localhost:8080/api/user/info`, `jq -r .code`), String) |> strip
        @test code == "200"

        authuri = "localhost:8080/api/user/info?access_token=$jwt"
        code = read(pipeline(`curl -s $authuri`, `jq -r .code`), String) |> strip
        @test code == "200"
    catch e
        @error "test api for $kind failed: $e"
    finally
        println()
        kill(proc)
        clean()
    end
end

function test_and_checkout(kind::String)
    checkout()

    @info("Test for $kind prepare")
    run(`sed -i "s/default\ =\ \[\ \"mysql\"\ \]/default = [ \"$kind\" ]/g" Cargo.toml `)
    kind == "mysql" || run(pipeline(`cat .env `, `grep $kind`, `sed  's/\#//gw .env'`))

    json = read(pipeline(`cat template.json`, `grep -v db`), String)
    db = read(pipeline(`cat template.json`, `grep $kind`, `sed 's/\/\/ //g'`), String)
    json2 = """{$db\n$(json[2:end])"""
    run(pipeline(`echo $json2`, `jq .`, `sed 'w template.json'`))
    
    @info("Test for $kind ..")
      
    try
        test(kind)
        @warn("Test for $kind Ok\n")
    catch e
        @error("Test for $kind failed: $e\n")
    end

    checkout()
end

test_and_checkout("mysql")
test_and_checkout("postgres")
test_and_checkout("sqlite")
