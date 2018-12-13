# hero-api

This project is a simple webserver built with [Rust](https://www.rust-lang.org)
and [Rocket](https://rocket.rs).
It comes with a simple unit test and TravisCI integration.

## Getting Started

### Setting up the environment
To download the latest nightly build of Rust and its package manager, [cargo](https://doc.rust-lang.org/cargo/), run
```bash
make setup
```

### Download Postgres
On macOS, you can run 
```bash
brew install postgresql
```

### Running the server
To start the server
```bash
# Start the server!
make

# Hit an endpoint!
curl http://localhost:8000/hello/John/58
```

### Running tests
```bash
make test
```

### Formatting code
First, clone [rustfmt](https://github.com/rust-lang/rustfmt) and run `cargo install --path . --force`.
That should install `rustfmt` in your `~/.cargo/bin`.
You should be able to format files after that:
```bash
make fmt
```

