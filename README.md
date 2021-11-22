# Rust Learning Demo App

A small app written in Rust using Actix Web and sqlx. This is my adventure in learning Rust, so it is not intended to 
be production-ready code.

## Development

### Requirements

- Rust
- sqlx-cli crate
- PostgreSQL or Docker

### Setup

If you do not have PostgreSQL already running on your machine, a `docker-compose.yml` file is included here that will
get PostgreSQL up and running for you. To start it, simply run:

```shell
docker-compose up -d
```

And to stop it, run:

```shell
docker-compose down
```

Next, set up a `.env` file in the root of the cloned repository. This is where you can define your database
configuration and log level:

```dotenv
RUST_LOG=debug
# DB_HOST=localhost (default)
# DB_PORT=5432 (default)
# DB_USER=identity (default)
DB_PASSWORD=pgdev
# DB_NAME=identity (default)
```

Now you can run the `migrate.sh` script to initialize the database:

```shell
./migrate.sh init
```

### Running

This can vary according to what IDE you are using, so we will stick to the `cargo` commands here. To build the
project, you can run the following from the project root:

```shell
cargo build
```

And then to start the server:

```shell
cargo run
```