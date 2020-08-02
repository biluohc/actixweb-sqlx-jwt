#!/bin/bash
# https://docs.julialang.org/en/v1.2/manual/faq/#How-do-I-catch-CTRL-C-in-a-script?-1
#= 
exec julia --color=yes -e 'include(popfirst!(ARGS))' \
    "${BASH_SOURCE[0]}" "$@" =#

checkout() = run(`git checkout -f .env Cargo.toml template.json`)

bd = read(pipeline(`cargo metadata --format-version 1`, `jq -r .target_directory`), String) |> strip
clean() = run(`rm -frv $bd/release/actixweb-sqlx-jwt`)

function test(kind::String)
    clean()

    run(`cargo build --release`)
    proc = run(`$bd/release/actixweb-sqlx-jwt -v`, wait=false)
    sleep(1)

    success(`curl 0.0.0.0:8080/static`) || @warn("Test for $kind failed")
    kill(proc)

    clean()
end

function test_and_checkout(kind::String)
    checkout()

    @info("Test for $kind prepare")
    run(`sed -i "s/default\ =\ \[\ \"mysql\"\ \]/default = [ \"$kind\" ]/g" Cargo.toml `)
    kind == "mysql" || run(pipeline(`cat .env `, `grep $kind`, `sed  's/\#//gw .env'`))

    json = read(pipeline(`cat template.json`, `grep -v sql`), String)
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
