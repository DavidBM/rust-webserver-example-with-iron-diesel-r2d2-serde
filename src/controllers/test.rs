import_controller_generic_requeriments!();

pub fn ping(_: &mut Request) -> IronResult<Response>{
	response_ok_text("pong")
}

pub fn read_login_user(req: &mut Request) -> IronResult<Response> {
	println!("{:?}", req.get_user_data());
	response_ok(req.get_user_data())
}
