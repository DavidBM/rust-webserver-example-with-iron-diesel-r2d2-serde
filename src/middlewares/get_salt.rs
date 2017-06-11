use std::env;
use slog::*;

use iron::{typemap, BeforeMiddleware};
use iron::prelude::*;

#[derive(Clone)]
pub struct GetSaltMiddleware {
	pub logger: Logger,
	salt: String
}

impl GetSaltMiddleware {
	pub fn new (logger: &Logger) -> GetSaltMiddleware{
		let salt = env::var("PASSWORD_SALT").expect("PASSWORD_SALT must be set");

		GetSaltMiddleware {logger: logger.new(o!("module" => "GetSaltMiddleware")), salt: salt}
	}
}

pub struct Value(String);

impl typemap::Key for GetSaltMiddleware { type Value = Value; }

impl BeforeMiddleware for GetSaltMiddleware {
	fn before(&self, req: &mut Request) -> IronResult<()> {
		req.extensions.insert::<GetSaltMiddleware>(Value(self.salt.clone()));
		Ok(())
	}
}

pub trait GetSaltReqExt {
	fn get_salt(&self) -> &String;
}

impl <'a, 'b>GetSaltReqExt for Request <'a, 'b> {
	fn get_salt(&self) -> &String {
		let &Value(ref salt) = self.extensions.get::<GetSaltMiddleware>().unwrap();

		salt
	}
}
