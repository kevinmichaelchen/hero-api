# hero-api

This project is a simple webserver built with [Rust](https://www.rust-lang.org)
and [Rocket](https://rocket.rs).
It comes with a simple unit test and TravisCI integration.

## Getting Started

### First time setup (just to run)
#### Setting up the environment
To download the latest nightly build of Rust and its package manager, [cargo](https://doc.rust-lang.org/cargo/), run
```bash
make setup
```

#### Setting up the database
This demo uses MySQL, since it's what I know best.
Download MySQL, log in as root, (usually passwordless), with:
```
mysql -uroot
```

Then, in the MySQL REPL, run:
```
mysql> CREATE DATABASE hero;
Query OK, 1 row affected (0.07 sec)

mysql> CREATE USER IF NOT EXISTS 'hero'@'localhost' IDENTIFIED BY 'hero';
Query OK, 0 rows affected (0.07 sec)

mysql> GRANT ALL ON hero.* TO 'hero'@'localhost';
Query OK, 0 rows affected (0.11 sec)

mysql> exit
Bye
```

You can log in as the `hero` user you just created with:
```
mysql -uhero -phero -Dhero
```

#### Setting up Diesel
[Diesel](http://diesel.rs) is our ORM. It manages database interactions and migrations.
```
cargo install diesel_cli --no-default-features --features "postgres mysql"
```

### First time setup (to contribute)
#### 
When I was first setting up Diesel, I had to have it generate a `./migrations` directory with:
```bash
env DATABASE_URL=mysql://hero:hero@localhost/hero diesel setup
```

Since we already have this directory, you won't have to run this again.

Whenever you want to make schema changes, you can tell Diesel to generate both an "up" script and a "down" script.
The former will get run when you migrate forward; the latter will be used to revert schema and database changes. 

```
# create migration scripts (both up and down)
diesel migration generate create_hero_table

# list migrations
env DATABASE_URL=mysql://hero:hero@localhost/hero diesel migration list

# run migrations
env DATABASE_URL=mysql://hero:hero@localhost/hero diesel migration run
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

