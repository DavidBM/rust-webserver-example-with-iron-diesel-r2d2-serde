use router::Router;
use mount::Mount;
use iron::prelude::Chain;
use slog::Logger;

use controllers::test as TestController;
use controllers::login as LoginController;
use controllers::user as UserController;
use controllers::post as PostController;

use middlewares::LoginMiddleware;
use middlewares::GetSaltMiddleware;

macro_rules! declare_multiple_endpoints {
	($main_router:expr, $mount_route:expr, $( $name:expr => $method:ident ; $route:expr ; [$($middleware_before:expr),*] => $handler:expr => [$($middleware_after:expr),*]),*) => {
		{
			let mut sub_router = Router::new();
			$(
				{
					#![allow(unused_mut)]
					let mut chain = Chain::new($handler);

					$(
						chain.link_before($middleware_before);
					)*

					$(
						chain.link_before($middleware_after);
					)*

					sub_router.$method($route, chain, $name);
				}
			)*
			$main_router.mount($mount_route, sub_router);
		}
	}
}

/*
	Declare routes with middlewares before and after.
	"<route name>" => <method>; <route>; [<array of before middlewares>] => <controller method> => [<array of after middlewares>]
*/
pub fn declare_endpoints(routes: &mut Mount, logger: &Logger) {
	let loggin = LoginMiddleware::new(&logger);
	let salt = GetSaltMiddleware::new(&logger);

	declare_multiple_endpoints!(
		routes, "/", //Declares the main route of the endpoints 
		"ping" => get; "/ping"; [] => TestController::ping => [],
		"read_login_user" => get; "/read_login_user"; [loggin.clone()] => TestController::read_login_user => [],
		"login" => post; "/login"; [salt.clone()] => LoginController::login => [],
		"register" => post; "/register"; [salt.clone()] => LoginController::register => []
	);

	declare_multiple_endpoints!(
		routes, "/user/", //All this endpoints will be inside of the route /user/
		"get_me" => get; "/me"; [loggin.clone()] => UserController::get_me => [],
		"get_user_posts" => get; "/me/post"; [loggin.clone()] => UserController::get_user_posts => [],
		"get_user" => get; "/:id"; [] => UserController::get => [],
		"delete_user" => delete; "/:id"; [] => UserController::delete => [],
		"update_user" => put; "/:id"; [] => UserController::update => []
	);

	declare_multiple_endpoints!(
		routes, "/post/",
		"create_post" => post; "/"; [loggin.clone()] => PostController::create => []
	);
}
