use std::collections::HashSet;
use std::env;

use iron::prelude::*;
use mount::Mount;
use slog::Logger;

use http_adaptor::declare_endpoints;

use middlewares::DieselMiddleware;
use middlewares::LoggerMiddleware;
use iron_cors::CorsMiddleware;


pub struct HttpAdaptor {
	logger: Logger
}

impl HttpAdaptor {
	pub fn new(logger: &Logger) -> HttpAdaptor {
		HttpAdaptor {logger: logger.new(o!("module" => "HttpAdaptor"))}
	}

	pub fn declare_endpoints(&mut self) -> Mount{
		let mut routes = Mount::new();

		declare_endpoints(&mut routes, &self.logger);

		routes
	}

	pub fn create_chain(&self, routes: Mount) -> Chain {
		let mut chain = Chain::new(routes);

		self.add_default_middlewares(&mut chain);

		chain
	}

	fn add_default_middlewares(&self, chain: &mut Chain) {
		let db_pool_middleware = DieselMiddleware::new(&self.logger);
		let logger_middleware = LoggerMiddleware::new(&self.logger);

		chain.link_before(logger_middleware);
		chain.link_before(db_pool_middleware);

		chain.link_around(self.create_cors_middleware());
	}

	pub fn start_http(&self, chain: Chain, host: &str, port: &str) {
		let address = format!("{}:{}", host, port);
		
		{
			info!(self.logger, "Server Running"; o!("address" => address.clone()));
		}

		Iron::new(chain).http(address).unwrap();
	}
	#[allow(dead_code)]
	fn create_cors_middleware(&self) -> CorsMiddleware {
		let domains = env::var("ALLOW_CORS_DOMAINS").expect("ALLOW_CORS_DOMAINS must be set");
		let domains = domains.split(",").map(ToString::to_string).collect::<HashSet<String>>();

		CorsMiddleware::with_whitelist(domains)
	}
}
