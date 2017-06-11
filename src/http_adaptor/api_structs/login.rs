#[derive(Debug, Deserialize)]
pub struct Login {
	pub user_or_email: String,
	pub password: String
}

