# Pahtecatl

Pahtecatl is a simple open source Gateway API to coordinate microservices access, developed with [Rust](https://www.rust-lang.org/).

## Setup

- [Rust](https://www.rust-lang.org/learn/get-started) (~ 1.80)

## Installation

To download and build the project, open a terminal and execute:

```sh
git clone https://github.com/huasteka/pahtecatl.git
cd pahtecatl
cargo build
```

__Cargo__ is the dependency manager that is installed together with __Rust__. 

## Configuration

To configure the API service proxy modify `config/default.toml` file:

```toml
[proxies.$serviceName]
target_service = $serviceURL
target_port = $servicePort
```

## Tests

To execute all tests, open a terminal and execute:

```sh
cargo test
```

## Run

To run the application, open a terminal and execute:

```sh
cargo run
```

The application will be served at `http://localhost:9705`.

## License

Pahtecatl is Copyright © 2024 Huasteka.

It is free software, and may be redistributed under the terms specified in the [LICENSE.md](LICENSE.md)
