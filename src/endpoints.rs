use iron::prelude::*;
use router::Router;
use mount::Mount;

use db::*;
use test_controller;
use user_controller;

macro_rules! declare_endpoint {
	($router:expr, $pool:expr, $route:expr, $method:ident, $name:expr, $lambda:expr) => {
		{
			let query_pool = $pool.clone();
			$router.$method($route, move |req: &mut Request| -> IronResult<Response> {
				$lambda(req, query_pool.get().unwrap())
			}, $name);
		}
	}
}

macro_rules! declare_multiple_endpoints {
	($pool:expr, $mainRouter:expr, $mount_route:expr, $( $name:expr => $method:ident, $route:expr, $handler:expr ),*) => {
		{
			let mut sub_router = Router::new();
			$(
				declare_endpoint!(sub_router, $pool, $route, $method, $name, $handler);
			)*
			$mainRouter.mount($mount_route, sub_router);
		}
	}
}

pub fn declare_endpoints(pool: DbConnectionPool) -> Mount {
	let mut routes = Mount::new();

	declare_multiple_endpoints!(
		pool, routes, "/",
		"ping" => get, "/ping", test_controller::ping
	);

	declare_multiple_endpoints!(
		pool, routes, "/user/",
		"get_user" => get, "/:id", user_controller::get_user,
		"create_user" => post, "/:id", user_controller::create_user,
		"delete_user" => delete, "/:id", user_controller::delete_user,
		"update_user" => put, "/:id", user_controller::update_user,
		"get_all_user" => get, "/", user_controller::get_all_users
	);

	routes
}
