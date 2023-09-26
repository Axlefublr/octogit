use super::input::UserGlyphs;

mod default;

pub struct ChosenGlyphs {
	pub unpushed: String,
	pub unstaged: String,
	pub added: String,
	pub staged: String,
	pub modified: String,
	pub renamed: String,
	pub deleted: String,
	pub staged_deleted: String,
}

impl ChosenGlyphs {
	pub fn from(user: UserGlyphs) -> Self {
		let unpushed = handle_glyph(
			user.unpushed,
			user.ascii_symbols,
			default::UNPUSHED,
			Some(default::UNPUSHED_NERD),
		);
		let renamed = handle_glyph(
			user.renamed,
			user.ascii_symbols,
			default::RENAMED,
			Some(default::RENAMED_NERD),
		);
		let added = handle_glyph(user.added, user.ascii_symbols, default::ADDED, None);
		let staged = handle_glyph(
			user.staged,
			user.ascii_symbols,
			default::STAGED,
			Some(default::STAGED_NERD),
		);
		let staged_deleted = handle_glyph(
			user.staged_deleted,
			user.ascii_symbols,
			default::DELETED,
			Some(default::DELETED_NERD),
		);
		let modified = handle_glyph(user.modified, user.ascii_symbols, default::MODIFIED, None);
		let deleted = handle_glyph(
			user.deleted,
			user.ascii_symbols,
			default::DELETED,
			Some(default::DELETED_NERD),
		);
		let unstaged = handle_glyph(user.unstaged, user.ascii_symbols, default::UNSTAGED, None);
		Self {
			unpushed,
			renamed,
			added,
			staged,
			staged_deleted,
			modified,
			deleted,
			unstaged,
		}
	}
}

fn handle_glyph(
	glyph: Option<String>,
	ascii_symbols: bool,
	ascii_default: char,
	nerd_default: Option<char>,
) -> String {
	if let Some(glyph) = glyph {
		glyph
	} else if ascii_symbols {
		ascii_default.to_string()
	} else if let Some(nerd_default) = nerd_default {
		nerd_default.to_string()
	} else {
		ascii_default.to_string()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use super::super::input::UserGlyphs;

	impl UserGlyphs {
		fn none(ascii_symbols: bool) -> Self {
			Self {
				ascii_symbols,
				unpushed: None,
				renamed: None,
				added: None,
				staged: None,
				staged_deleted: None,
				modified: None,
				deleted: None,
				unstaged: None,
			}
		}
	}

	#[test]
	fn nerd() {
		let chosen = ChosenGlyphs::from(UserGlyphs::none(false));
		assert_eq!(default::UNPUSHED_NERD.to_string(), chosen.unpushed);
		assert_eq!(default::RENAMED_NERD.to_string(), chosen.renamed);
		assert_eq!(default::ADDED.to_string(), chosen.added);
		assert_eq!(default::STAGED_NERD.to_string(), chosen.staged);
		assert_eq!(default::DELETED_NERD.to_string(), chosen.staged_deleted);
		assert_eq!(default::MODIFIED.to_string(), chosen.modified);
		assert_eq!(default::DELETED_NERD.to_string(), chosen.deleted);
		assert_eq!(default::UNSTAGED.to_string(), chosen.unstaged);
	}

	#[test]
	fn ascii() {
		let chosen = ChosenGlyphs::from(UserGlyphs::none(true));
		assert_eq!(default::UNPUSHED.to_string(), chosen.unpushed);
		assert_eq!(default::RENAMED.to_string(), chosen.renamed);
		assert_eq!(default::ADDED.to_string(), chosen.added);
		assert_eq!(default::STAGED.to_string(), chosen.staged);
		assert_eq!(default::DELETED.to_string(), chosen.staged_deleted);
		assert_eq!(default::MODIFIED.to_string(), chosen.modified);
		assert_eq!(default::DELETED.to_string(), chosen.deleted);
		assert_eq!(default::UNSTAGED.to_string(), chosen.unstaged);
	}
}
