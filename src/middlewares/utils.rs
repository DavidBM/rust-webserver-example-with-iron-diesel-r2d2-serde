use std::error;
use std::fmt;

#[derive(Debug)]
pub enum MiddlewareErrorTypes {
	AuthorizationError
}

impl fmt::Display for MiddlewareErrorTypes {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			MiddlewareErrorTypes::AuthorizationError => write!(f, "Authorization failed"),
		}
	}
}

impl error::Error for MiddlewareErrorTypes {
	fn description(&self) -> &str {
		match *self {
			MiddlewareErrorTypes::AuthorizationError => "Authorization failed",
		}
	}

	fn cause(&self) -> Option<&error::Error> {
		match *self {
			MiddlewareErrorTypes::AuthorizationError => None
		}
	}
}
