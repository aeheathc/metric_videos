# Metric Videos
actix-web + React app showing a dashboard of real-time video streaming metrics

## How to run
- Run `cargo run` in the project root with Rust installed. By default it will be available at `http://localhost:8000`
- You can also build it and run the executable in any location. Use the `--help` option to see how to tell it where to find the "data" directory, so it can load the static resources, config file, etc. You can also change the port it listens on.

## Other things you can do with the code
- Run `cargo test` to run the unit tests
- Run `cargo clippy` to run the linter
- Run `cargo doc` to build HTML docs from the "doc comments" found in the source. The docs will be available at `target/doc/metric_videos/index.html`