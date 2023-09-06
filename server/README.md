# Abineo Analytics Server

Visit [abineo.swiss/analytics](https://abineo.swiss/analytics) for more Information.

## Developer Setup

Start the database with [docker compose](https://docs.docker.com/engine/install/).

```sh
docker compose up
```

This should create a [PostgreSQL](https://postgresql.org) database with the [TimescaleDB](https://timescale.com)
extension installed.

Start the server using [cargo](https://rustup.rs/).

```sh
cargo run
```

## License

[☕ Coffee License 2.0](https://coffee-license.org/v2.0).
