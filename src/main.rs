use ansi_term::{ANSIString, ANSIStrings, Style};
use std::env;

mod colors {
	use ansi_term::Color;
	pub const PINK: Color = Color::RGB(0xFF, 0xAF, 0xD7);
	pub const YELLOW: Color = Color::RGB(0xFF, 0xD7, 0x5F);
	// pub const PURPLE: Color = Color::RGB(0xAF, 0x87, 0xFF);
	// pub const GREY: Color = Color::RGB(0x87, 0x87, 0x87);
	// pub const GREEN: Color = Color::RGB(0x87, 0xFF, 0x5F);
	// pub const CYAN: Color = Color::RGB(0x00, 0xD7, 0xFF);
	// pub const RED: Color = Color::RGB(0xFF, 0x00, 0x5F);
	// pub const ORANGE: Color = Color::RGB(0xFF, 0x87, 0x00);
	// pub const REDIS: Color = Color::RGB(0xFF, 0x87, 0x87);
	// pub const WHITE: Color = Color::RGB(0xD4, 0xBE, 0x98);
}

fn main() {
	let prompt_elements: &[ANSIString<'static>] = &[
		pwd(colors::PINK.bold()),
		Style::default().paint(" "),
		colors::YELLOW.paint("\n╰─ "),
	];
	print!("{}", ANSIStrings(prompt_elements));
}

fn pwd(color: Style) -> ANSIString<'static> {
	let pwd = env::current_dir().expect("current directory should've loaded in");
	let pwd = pwd
		.file_name()
		.expect("current directory shouldn't've ended in ..")
		.to_str()
		.expect("current directory should've been valid unicode")
		.to_owned();
	color.paint(pwd)
}
