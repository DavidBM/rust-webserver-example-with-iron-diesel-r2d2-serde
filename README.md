# Rust webserver example backend

<!-- MarkdownTOC autolink=true autoanchor=true bracket=round depth=0 -->

- [Structure](#structure)
- [Dependencies](#dependencies)
- [Before run](#before-run)
- [Running \(develop\)](#running-develop)
- [Deploy](#deploy)
- [Creating new database migrations](#creating-new-database-migrations)
- [Running migrations](#running-migrations)
- [Adding new endpoints](#adding-new-endpoints)
- [Recommended packages for working in Rust & sublime text editor](#recommended-packages-for-working-in-rust--sublime-text-editor)
- [TODO](#todo)

<!-- /MarkdownTOC -->

Example of how to build a http json backend in rust. The code is split in controllers/models. For more complexity new abastraction shluld be created. 

Main components:
- slog: log system
- r2d2: database connection pool
- diesel: ORM
- iron: http framework
- serde: json en/decoder

Other components/characteristics
- Database password hash
- CORS support
- Login with JSON Webtokens
- Middlewares

<a name="structure"></a>
## Structure

Routes are declared in `src/http_adaptor/endpoints`. Some macros are used for automate the code there.

The Diesel (PostgreSQL) ORM connections are pooled with r2d2.

The output is a JSON version of the database model using Serde json.

This code use macros in several places, use `cargo expand` (`cargo install cargo-expand`) for see the final code.

<a name="dependencies"></a>
## Dependencies

- rust and cargo (Install using [https://www.rustup.rs/](https://www.rustup.rs/))
- diesel-cli - Install: `cargo install diesel_cli`
- PostgreSQL (accesible from the server. Can be local or remote)

Compiled using Rust `rustc 1.16.0 (30cf806ef 2017-03-10)` stable version. 

<a name="before-run"></a>
## Before run

- Enter in the git folder `cd templic-backend`
- Update the database connection data in the file `.env`
- `diesel migration run`

<a name="running-develop"></a>
## Running (develop)

- `cargo run`

<a name="deploy"></a>
## Deploy

- Copy the executable compiled with `cargo build --release` in the server (Sign the executable and verify).
- Copy the `.env` of the repository next to the executable. 
- Fill the data of `.env`.
- Execute the executable. 

<a name="creating-new-database-migrations"></a>
## Creating new database migrations

- `diesel migration generate <migration name>`
- fill the files `up.sql` and `down.sql`
- check with `diesel migration run`
- check again with `diesel migration redo`
- Execute `cargo build` for checking types in the code. (Types depend of the actual types in the database, and the compiler use that information for checking compatibility with code types)

If there isn't any errors, you finish! :)

<a name="running-migrations"></a>
## Running migrations

- Install [dependencies](#dependencies)
- execute `diesel migration run`

<a name="adding-new-endpoints"></a>
## Adding new endpoints 

- Add the function to the controller or create a new controller in `src/controllers`.
	+ If a new controller is created, don't forget to add it in `src/controllers/mod.rs`.
- Add the function to `src/http_adaptor/endpoints.rs`.
	+ Maybe you need to add `use controllers::<controller file name>;` at the beginning of the file.
- Fill the controller function
- Test it

<a name="recommended-packages-for-working-in-rust--sublime-text-editor"></a>
## Recommended packages for working in Rust & sublime text editor

- Sublime packages
	+ rust enhanced
	+ anaconda_rust
	+ sublimeLinter-contrib-rustc
- `cargo install cargo-expand`
- `cargo install cargo-watch`

<a name="todo"></a>
## TODO

- [ ] Create get /post/:id
- [ ] Create delete /post/:id
- [ ] Create put /post/:id
- [ ] Tokens MUST change every login
- [ ] Add caducity to tokens
- [ ] Add token_version to user table for cancelling tokens for users
- [ ] Improve return macros for showing the internal error (like `response_bad_request`)
- [ ] Add permissions for delete & update user (only the same user can do it)
- [ ] Delete user must delete all the posts
- [ ] Logs in queries should identify the query action

Blocked

- [ ] Update `jsonwebtoken` and remove `rustc-serialize` (it breaks `cargo expand`)
- [ ] Update to better security `argon2rs` (using at least 10 passes and all possible lanes)
- [ ] Help in `iron-cors` and reactivate the crate in the code
- [ ] `base64`. Maintain update.
- [ ] Debug the SQL of the update methods
