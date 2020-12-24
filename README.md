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

Start server for development:
> cargo run --bin service

Tests webservice:
> curl -s http://localhost:8000/edges

> curl -s -X PUT "http://localhost:8000/edge/172.188.1.235/desc"

> curl -s http://localhost:8000/news/e03d0c3c-70b0-9ad3-37e9-50815fbc7ddf | jq .

> curl -s -X DELETE "http://localhost:8000/edge/e03d0c3c-70b0-9ad3-37e9-50815fbc7ddf"

