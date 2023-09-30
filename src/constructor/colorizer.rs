use super::input::UserColors;
use ansi_term::Color;

mod default;

pub struct ChosenColors {
	pub stashed: Color,
	pub unpulled: Color,
	pub unpushed: Color,
	pub renamed: Color,
	pub staged: Color,
	pub added: Color,
	pub staged_deleted: Color,
	pub modified: Color,
	pub unstaged: Color,
	pub deleted: Color,
}

impl ChosenColors {
	pub fn from(user: UserColors) -> (Self, Vec<String>) {
		let mut errors = vec![];

		let all_commits = handle_color(user.all_commits, default::YELLOW, &mut errors);
		let all_staged = handle_color(user.all_staged, default::GREEN, &mut errors);
		let all_unstaged = handle_color(user.all_unstaged, default::CYAN, &mut errors);

		let stashed = handle_color(user.stashed, all_commits, &mut errors);
		let unpulled = handle_color(user.unpulled, all_commits, &mut errors);
		let unpushed = handle_color(user.unpushed, all_commits, &mut errors);

		let renamed = handle_color(user.renamed, all_staged, &mut errors);
		let staged = handle_color(user.staged, all_staged, &mut errors);
		let added = handle_color(user.added, all_staged, &mut errors);
		let staged_deleted = handle_color(user.staged_deleted, all_staged, &mut errors);

		let modified = handle_color(user.modified, all_unstaged, &mut errors);
		let unstaged = handle_color(user.unstaged, all_unstaged, &mut errors);
		let deleted = handle_color(user.deleted, all_unstaged, &mut errors);

		let chosen = Self {
			stashed,
			unpulled,
			unpushed,
			renamed,
			staged,
			added,
			staged_deleted,
			modified,
			unstaged,
			deleted,
		};
		(chosen, errors)
	}
}

fn handle_color(color: Option<String>, default: Color, errors: &mut Vec<String>) -> Color {
	let color = match color {
		Some(color) => color,
		None => return default,
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
				None => return err_msg,
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
