<h1> RustRestAPI </h1>
API using Rust

Based from project on:

https://medium.com/@ilegra/building-a-microservice-with-rust-ef9641cf2331
https://github.com/diegopacheco/rust-playground/tree/master/rust-microservice

***
<h2> Create Database using Docker Container </h2>

<h3> Setting up database: </h3>

> docker-compose build

> docker-compose up

Use adminer for database administer using web brownser:

> http://127.0.0.1:8080/

* The database, password and user have been set from the docker compose file.

<h3> Migrations: </h3>
    Create database tables, index etc

> cargo run --bin migrations

***
<h3>  Running webservice </h3>

> curl -s http://localhost:8000/news | jq .

> curl -s -X PUT "http://localhost:8000/news/facebook/faceboo.com"

> curl -s http://localhost:8000/news/0dae39e4-fca2-b076-4f88-617dd3352d11 | jq .

> curl -s -X DELETE "http://localhost:8000/news/0dae39e4-fca2-b076-4f88-617dd3352d11"

