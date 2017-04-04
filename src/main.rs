extern crate iron;
extern crate router;
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate dotenv;

use iron::prelude::*;
use iron::status;
use router::Router;

use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use dotenv::dotenv;
use std::env;

fn main() {
	dotenv().ok();
	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

	let config = r2d2::Config::default();
	let manager = ConnectionManager::<PgConnection>::new(database_url);
	let pool = r2d2::Pool::new(config, manager).expect("Failed to create diesel pool.");

	let mut router = Router::new();
	
	router.get("/:query", move |req: &mut Request| -> IronResult<Response> {
		let connection = pool.get();
		
		let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
		
		println!("{:?}", req);
		
		//Ok(Response::with((status::Ok, *query)))
		
		Ok(Response::with((status::Ok, connection.is_ok().to_string())))
	}, "query");

	Iron::new(router).http("localhost:3000").unwrap();
}