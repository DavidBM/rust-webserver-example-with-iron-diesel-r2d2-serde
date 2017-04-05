# Rust Web server example using Iron, R2d2 Diesel and Serde json 

This is an example of how to create a simple webserver using the framework Iron and Diesel. The Diesel connection are pooled with r2d2, meaning that all the Iron threats can do queries at the same time.

The output is a JSON version of the database model using Serde json.

There is tons of optimizations that can be done, but I think is a good start. 

This code use macros in several places, use `cargo expand` (`cargo install cargo-expand`) for see the final code.

## Dependencies

- rust and cargo (Install using [https://www.rustup.rs/](https://www.rustup.rs/))
- diesel-cli - Install: `cargo install diesel_cli`
- PostgreSQL

Compiled using Rust `rustc 1.16.0 (30cf806ef 2017-03-10)` stable version. 

## Before run

- Update the database connection data in the file `.env`
- `cd rust-webserver-demo`
- `diesel migration run`

## Running

- `cargo run`