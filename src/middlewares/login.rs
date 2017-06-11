use std::env;

use iron::{typemap, BeforeMiddleware, status};
use iron::error::IronError;
use iron::headers::{Authorization, Bearer};
use iron::prelude::*;

use jwt::{decode, Algorithm};
use dal::models::user::User;
use slog::Logger;

use middlewares::MiddlewareErrorTypes;
use middlewares::diesel_pool::DieselReqExt;


#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Token {
	pub user_id: i32
}

#[derive(Clone)]
pub struct LoginMiddleware {
	logger: Logger
}

impl LoginMiddleware {
	pub fn new(logger: &Logger) -> LoginMiddleware {
		let logger = logger.new(o!("module" => "LoginMiddleware"));
		LoginMiddleware {logger: logger}
	}
}

pub struct Value(User);

impl typemap::Key for LoginMiddleware { type Value = Value; }

impl BeforeMiddleware for LoginMiddleware {
	fn before(&self, req: &mut Request) -> IronResult<()> {

		let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

		let token = req.headers.get::<Authorization<Bearer>>();

		if let Some(&Authorization(ref bearer)) = token {
			match decode::<Token>(&bearer.token, secret.as_ref(), Algorithm::HS512) {
				Ok(user) => {
					let connection = req.get_db_conn();

					let user = some_or_return!(
						User::get_by_id(user.claims.user_id, &connection, &self.logger),
						Err(IronError::new(MiddlewareErrorTypes::AuthorizationError, status::Unauthorized))
					);

					info!(self.logger, "Request authorized"; "user_id" => user.id);
					
					req.extensions.insert::<LoginMiddleware>(Value(user));

					Ok(())
				},
				Err(error) => {
					info!(self.logger, "Request unauthorized"; "reason" => "JWT error", "details" => format!("{:?}", error));
					Err(IronError::new(error, status::Unauthorized))
				}
			}
		}
		else {
			info!(self.logger, "Request unauthorized"; "reason" => "no bearer token found", "details" => format!("{:?}", token));
			Err(IronError::new(MiddlewareErrorTypes::AuthorizationError, status::Unauthorized))
		}
	}
}

pub trait LoginReqExt {
	fn get_user_data(&self) -> &User;
}

impl <'a, 'b>LoginReqExt for Request <'a, 'b> {
	fn get_user_data(&self) -> &User {
		let &Value(ref user) = self.extensions.get::<LoginMiddleware>().expect("LoginMiddleware not in the chain");
		user
	}
}
