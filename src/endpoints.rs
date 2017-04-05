use iron::prelude::*;
use router::Router;

use db::*;
use test_controller;
use user_controller;

macro_rules! declare_endpoint {
	($router:expr, $route:expr, $name:expr, $lambda:expr, $pool:expr) => {
		{
			let query_pool = $pool.clone();
			$router.get($route, move |req: &mut Request| -> IronResult<Response> {
				$lambda(req, query_pool.get().unwrap())
			}, $name);
		}
	}
}

pub fn declare_endpoints(pool: DbConnectionPool) -> Router {
	let mut router = Router::new();

	//Test controller
	declare_endpoint!(router, "/ping", "ping", test_controller::ping, pool);

	//User controller
	declare_endpoint!(router, "/user/:id", "get_user", user_controller::get_user, pool);

	router
}
