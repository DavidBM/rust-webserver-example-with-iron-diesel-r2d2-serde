extern crate iron;
extern crate router;
extern crate mount;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate dotenv;
extern crate chrono;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_json;
extern crate iron_cors;
extern crate jsonwebtoken as jwt;
extern crate rustc_serialize;
extern crate argon2rs;
extern crate base64;

#[macro_use]
mod utils;
mod dal;
mod controllers;
mod http_adaptor;
mod middlewares;

use dotenv::dotenv;
use http_adaptor::HttpAdaptor;
use utils::logger_factory;

fn main() {
	dotenv().ok();

	let logger = logger_factory();

	let mut http_server = HttpAdaptor::new(&logger);

	let routes = http_server.declare_endpoints();
	let chain = http_server.create_chain(routes);


	http_server.start_http(chain, "localhost", "3000");
}
