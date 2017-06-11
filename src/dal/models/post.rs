use dal::models::user::User;
use dal::db_schema::*;
use diesel;
use diesel::prelude::*;
use middlewares::DieselConnection;
use slog::Logger;

#[derive(Debug, Insertable, Deserialize)]
#[table_name="posts"]
pub struct NewPost {
	pub user_id: Option<i32>,
	pub title: String,
	pub content: String
}


#[derive(Clone, Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User, foreign_key="user_id")]
pub struct Post {
	pub id: i32,
	pub user_id: i32,
	pub title: String,
	pub content: String
}

impl Post {
	pub fn create(post: &NewPost, connection: &DieselConnection, logger: &Logger) -> Result<Post, diesel::result::Error> {
		let statement = diesel::insert(post)
		.into(posts::table);

		info!(logger, "Executing Query"; "query" => debug_sql!(statement), "post" => format!("{:?}", post));

		let new_post = statement.get_result::<Post>(&**connection);

		match new_post {
			Ok(new_post) => Ok(new_post),
			Err(error) => {
				warn!(logger, "Error creating post"; o!("error" => format!("{:?}", error)));
				Err(error)
			},
		}
	}

	pub fn get_post_from_user(user_id: i32, connection: &DieselConnection, logger: &Logger) -> Vec<Post> {
		let statement = posts::table.filter(posts::user_id.eq(user_id));

		info!(logger, "Executing Query"; "query" => debug_sql!(statement), "user_id" => format!("{:?}", user_id));

		let posts: Result<Vec<Post>, _> = statement.load::<Post>(&**connection);

		match posts {
			Ok(posts) => posts,
			Err(error) => {
				warn!(logger, "Error creating post"; o!("error" => format!("{:?}", error)));
				Vec::new()
			}
		}
	}
}
