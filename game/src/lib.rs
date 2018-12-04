extern crate libc;

use libc::{int32_t, c_char};
use std::ffi::{CString, CStr};

pub struct Game {
	name: String,
	n: i32,
}

impl Game {
	fn new() -> Self {
		return Self {
			name: String::from("Number keeper"),
			n: 0,
		};
	}

	fn get_n(&self) -> i32 {
		return self.n;
	}

	fn set_n(&mut self, n: i32) {
		self.n = n;
	}

	fn get_name(&self) -> String {
		return self.name.clone();
	}

	fn set_name<'a>(&mut self, s: &'a str) {
		self.name = String::from(s);
	}
}

#[no_mangle]
pub extern fn game_new() -> *mut Game {
	return Box::into_raw(Box::new(Game::new()));
}

#[no_mangle]
pub extern fn game_free(ptr: *mut Game) {
	if ptr.is_null() {
		return;
	}

	unsafe {
		Box::from_raw(ptr);
	}
}


#[no_mangle]
pub extern fn game_get_n(ptr: *const Game) -> int32_t {
	let game = unsafe {
		assert!(!ptr.is_null());
		&*ptr
	};
	return game.get_n();
}

#[no_mangle]
pub extern fn game_set_n(ptr: *mut Game, new_n: i32) {
	let game = unsafe {
		assert!(!ptr.is_null());
		&mut *ptr
	};
	game.set_n(new_n);
}

#[no_mangle]
pub extern fn game_get_name(ptr: *mut Game) -> *const c_char {
	let game = unsafe {
		assert!(!ptr.is_null());
		&*ptr
	};

	let s = CString::new(game.get_name()).unwrap();
	return s.into_raw();
}

#[no_mangle]
pub extern fn game_set_name(ptr: *mut Game, name_ptr: *const c_char) {
	let game = unsafe {
		assert!(!ptr.is_null());
		&mut *ptr
	};

	let new_name = unsafe {
		assert!(!name_ptr.is_null());
		CStr::from_ptr(name_ptr).to_str().unwrap()
	};
	game.set_name(new_name);
}

#[no_mangle]
pub extern fn game_free_string(s: *mut c_char) {
	unsafe {
		if s.is_null() {
			return;
		}
		CString::from_raw(s);
	}
}
