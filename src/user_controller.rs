use iron::status;
use iron::prelude::*;
use router::Router;

use db::*;
use user_model::*;
use diesel::prelude::*;
use db_schema::users;

use serde_json;

pub fn get_user(req: &mut Request, connection: DbPooledConnection) -> IronResult<Response>{
	
	let ref user_id = req.extensions.get::<Router>().unwrap().find("id").unwrap_or("/");

	let user_id = user_id.parse::<i32>().unwrap();

	let user_data = users::table
	.filter(users::id.eq(user_id))
	.load::<User>(&*connection);

	let ref user_data = &user_data.unwrap()[0];

	let response_data = serde_json::to_string(user_data).unwrap();

	Ok(Response::with((
		status::Ok, 
		response_data
	)))
}

pub fn create_user(req: &mut Request, connection: DbPooledConnection) -> IronResult<Response> {
    unimplemented!();
}

pub fn delete_user(req: &mut Request, connection: DbPooledConnection) -> IronResult<Response> {
    unimplemented!();
}

pub fn update_user(req: &mut Request, connection: DbPooledConnection) -> IronResult<Response> {
    unimplemented!();
}

pub fn get_all_users(req: &mut Request, connection: DbPooledConnection) -> IronResult<Response> {
    unimplemented!();
}
