import_controller_generic_requeriments!();

use dal::models::post::*;

pub fn create(req: &mut Request) -> IronResult<Response> {
	let mut new_post = get_body_as!(NewPost, req, response_bad_request);
	let connection = req.get_db_conn();
	let logger = req.get_logger();

	new_post.user_id = Some(req.get_user_data().id);

	let post = ok_or_return!(
		Post::create(&new_post, &connection, &logger), 
		response_bad_request("Error creating the post")
	);

	response_ok(&json!({"post_id": post.id}))
}