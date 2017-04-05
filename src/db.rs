use diesel::pg::PgConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;
use std::env;

pub type DbPooledConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;
pub type DbConnectionPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Db {
	connection: DbConnectionPool
}

impl Db {
	pub fn new () -> Db{	
		let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

		let config = r2d2::Config::default();
		let manager = ConnectionManager::<PgConnection>::new(database_url);
		let connection = r2d2::Pool::new(config, manager).expect("Failed to create diesel pool.");

		Db {connection: connection}
	}

	pub fn get_pool(&self) -> DbConnectionPool {
		self.connection.clone()
	}
}
