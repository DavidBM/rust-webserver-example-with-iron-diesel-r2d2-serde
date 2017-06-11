macro_rules! ok_or_return {
	($item:expr, $result:expr) => {
		match $item {
			Ok(value) => value,
			Err(_) => return $result
		};
	}
}

macro_rules! some_or_return {
	($item:expr, $result:expr) => {
		match $item {
			Some(value) => value,
			None => return $result
		};
	}
}