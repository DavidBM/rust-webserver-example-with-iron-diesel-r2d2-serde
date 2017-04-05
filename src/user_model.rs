use chrono::NaiveDate;

#[derive(Debug, Queryable, Serialize)]
pub struct User {
	id: i32,
	name: String,
	email: String,
	password: String,
	created_at: NaiveDate,
}
