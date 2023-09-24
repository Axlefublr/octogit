use ansi_term::Color;
use clap::error;

mod default;

pub struct ChosenColors {
	pub unpushed: Color,
	pub unstaged: Color,
	pub added: Color,
	pub staged: Color,
	pub modified: Color,
	pub renamed: Color,
	pub deleted: Color,
	pub staged_deleted: Color,
}

impl ChosenColors {
	pub fn from(
		unpushed: Option<String>,
		all_staged: Option<String>,
		all_unstaged: Option<String>,
		unstaged: Option<String>,
		deleted: Option<String>,
		modified: Option<String>,
		added: Option<String>,
		staged: Option<String>,
		renamed: Option<String>,
		staged_deleted: Option<String>,
	) -> (Self, Vec<String>) {
		let mut errors = vec![];
		let unpushed = handle_color(unpushed, default::YELLOW, &mut errors);
		let all_staged = handle_color(all_staged, default::GREEN, &mut errors);
		let all_unstaged = handle_color(all_unstaged, default::CYAN, &mut errors);
		let unstaged = handle_color(unstaged, all_unstaged, &mut errors);
		let deleted = handle_color(deleted, all_unstaged, &mut errors);
		let modified = handle_color(modified, all_unstaged, &mut errors);
		let added = handle_color(added, all_staged, &mut errors);
		let staged = handle_color(staged, all_staged, &mut errors);
		let renamed = handle_color(renamed, all_staged, &mut errors);
		let staged_deleted = handle_color(staged_deleted, all_staged, &mut errors);
		let chosen = Self {
			unpushed,
			unstaged,
			added,
			staged,
			modified,
			renamed,
			deleted,
			staged_deleted
		};
		(chosen, errors)
	}
}

fn handle_color(color: Option<String>, default: Color, errors: &mut Vec<String>) -> Color {
	let color = match color {
		Some(color) => color,
		None => return default
	};
	match parse_color(color) {
		Ok(color) => color,
		Err(message) => {
			if !message.is_empty() {
				errors.push(message);
			}
			default
		}
	}
}

fn parse_color(color: String) -> Result<Color, String> {
	match color.to_lowercase().as_ref() {
		"black" => Ok(Color::Black),
		"red" => Ok(Color::Red),
		"green" => Ok(Color::Green),
		"yellow" => Ok(Color::Yellow),
		"blue" => Ok(Color::Blue),
		"purple" => Ok(Color::Purple),
		"cyan" => Ok(Color::Cyan),
		"white" => Ok(Color::White),
		hex => {
			let hex = hex.to_owned();
			let err_msg = Err(format!("hex color specified is incorrect: `{}`", &hex));
			let (red, green, blue) = match parse_hex(hex) {
				Some(rgb) => rgb,
				None => return err_msg
			};
			Ok(Color::RGB(red, green, blue))
		}
	}
}

fn parse_hex(color: String) -> Option<(u8, u8, u8)> {
	let red = u8::from_str_radix(&color[0..2], 16).ok()?;
	let green = u8::from_str_radix(&color[2..4], 16).ok()?;
	let blue = u8::from_str_radix(&color[4..6], 16).ok()?;
	Some((red, green, blue))
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn parse_color_base_16() {
		assert_eq!(Ok(Color::Red), parse_color("red".to_owned()));
	}

	#[test]
	fn parse_hex_from_str() {
		assert_eq!(
			Some((0x8a, 0x5c, 0xe7)),
			parse_hex("8a5ce7 hex color code first, then literally anything after".to_owned())
		);
	}
}
