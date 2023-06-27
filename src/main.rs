use std::env;
use ansi_term::{
	Style,
	ANSIString,
	ANSIStrings
};

mod my_colors {
	use ansi_term::Color;
	pub const PINK: Color = Color::RGB(0xFF, 0xAF, 0xD7);
}

fn main() {
	let prompt_elements: &[ANSIString<'static>] = &[
		pwd(my_colors::PINK.bold())
	];
	print!("{}", ANSIStrings(prompt_elements));
}

fn pwd(color: Style) -> ANSIString<'static> {
	let pwd = env::current_dir()
		.expect("current directory should've loaded in");
	let pwd = pwd
		.file_name()
		.expect("current directory shouldn't've ended in ..")
		.to_str()
		.expect("current directory should've been valid unicode")
		.to_owned();
	color.paint(pwd)
}
