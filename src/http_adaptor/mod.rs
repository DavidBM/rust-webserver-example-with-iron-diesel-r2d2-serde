pub mod adaptor;
pub mod endpoints;
pub mod api_structs;

use self::endpoints::declare_endpoints;

pub use self::api_structs as apis;
pub use self::adaptor::HttpAdaptor;
