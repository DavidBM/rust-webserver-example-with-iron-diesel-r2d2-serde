import_controller_generic_requeriments!();

use std::env;

use chrono::UTC;
use chrono::DateTime;
use argon2rs::argon2i_simple;
use base64::encode;
use jwt::{encode as encode_jwt, Algorithm, Header};

use dal::models::user::{User, NewUser};
use http_adaptor::apis::Login;
use middlewares::get_salt::GetSaltReqExt;

use middlewares::login::Token;

pub fn login(req: &mut Request) -> IronResult<Response>{
	let login_data = get_body_as!(Login, req, response_bad_request);
	let connection = req.get_db_conn();
	let logger = req.get_logger();

	let user_data = some_or_return!(
		User::get_user_by_email_or_name(&login_data, &connection, &logger), 
		response_not_found("User or password incorrect")
	);

	let encoded_password = hash_password(req, &user_data.created_at, &login_data.password);

	if user_data.password != encoded_password {
		return response_not_found("User or password incorrect");
	}

	let token = create_token(&user_data);

	response_ok_text(token)
}

pub fn register(req: &mut Request) -> IronResult<Response> {
	let mut user = get_body_as!(NewUser, req, response_bad_request);
	let connection = req.get_db_conn();
	let logger = req.get_logger();

	let created_at = UTC::now();

	user.password = hash_password(req, &created_at, &user.password);
	user.created_at = Some(created_at);

	let user_model = ok_or_return!(
		User::create(&user, &connection, &logger), 
		response_internal_server_error("Error saving the user into db")
	);

	response_ok(&json!({"user_id": user_model.id}))
}

fn hash_password(req: &mut Request, created_at: &DateTime<UTC>, login_password: &String) -> String {
	let salt = create_user_salt(&req.get_salt(), created_at);

	let hash = argon2i_simple(login_password.as_ref(), salt.as_ref());

	encode(&hash)
}

fn create_token(user_data: &User) -> String {
	let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

	let mut header = Header::default();
	header.alg = Algorithm::HS512;
	encode_jwt(header, &Token {user_id: user_data.id}, secret.as_ref()).unwrap()
}

fn create_user_salt(static_salt: &String, created_at: &DateTime<UTC>) -> String {
	let time = created_at.timestamp_subsec_millis().to_be();

	(
		time.count_ones() as u64 
		+ time.leading_zeros() as u64 
		+ time as u64 
		+ time.count_zeros().to_le() as u64 
		+ time.rotate_left(5) as u64 
	).to_string() 
	+ static_salt
}
