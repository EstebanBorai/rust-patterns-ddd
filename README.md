<div>
  <div align="center" style="display: block; text-align: center;">
    <img src="https://camo.githubusercontent.com/734a3468bce992fbc3b729562d41c92f4912c99a/68747470733a2f2f7777772e727573742d6c616e672e6f72672f7374617469632f696d616765732f727573742d6c6f676f2d626c6b2e737667" height="120" width="120" />
  </div>
  <h1 align="center">rust-patterns-ddd</h1>
  <h4 align="center">Rust Domain-Driven-Design (DDD) POC</h4>
</div>

## Motivation

Write a Rust Application following the Domain Driven Design pattern as
a proof of concept.

## Running Locally


1. Clone the repository locally

```shell
https://github.com/EstebanBorai/rust-patterns-ddd.git
```

2. Execute the `bin/dotenv` script to create a `.env` file
or copy the contents of the `.env.sample` file into a new file
with the name `.env`

3. Run the Docker instance using the `bin/docker-start` script

```shell
bin/docker-start
```

4. When the server is ready, run migrations to make sure every
table on the database is available at the moment of connecting and
executing queries.

```shell
bin/sqlx-cli migrate run
```

5. Install dependencies and execute the server

```bash
cargo run
```
