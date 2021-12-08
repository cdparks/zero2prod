# zero2prod

Following along with [Zero to Production in Rust](https://www.zero2prod.com/)

## Build

1. Initialize Postgres docker image and run migrations: `./scripts/init.sh`
2. Build server: `cargo build`

## Run

```bash
$ cargo run
```

This application uses the [bunyan log format][bunyan-format]. To pretty-print logs, pipe `cargo run` into the [node][node-bunyan] or [rust][bunyan-rs] version of the `bunyan` cli tool:

```bash
$ cargo run | bunyan
```

The log level can be controlled with the `RUST_LOG` environment variable.

## Test

Run `cargo test`.

[bunyan-format]: https://github.com/trentm/node-bunyan#log-record-fields
[node-bunyan]: https://github.com/trentm/node-bunyan
[bunyan-rs]: https://github.com/LukeMathWalker/bunyan
