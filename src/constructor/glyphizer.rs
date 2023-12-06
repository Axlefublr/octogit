use super::input::UserGlyphs;

mod default;

pub struct ChosenGlyphs {
    pub stashed: String,
    pub unpulled: String,
    pub unpushed: String,
    pub renamed: String,
    pub staged: String,
    pub added: String,
    pub staged_deleted: String,
    pub modified: String,
    pub unstaged: String,
    pub deleted: String,
}

impl ChosenGlyphs {
    pub fn from(user: UserGlyphs) -> Self {
        let stashed = handle_glyph(
            user.stashed,
            user.ascii_symbols,
            default::STASHED,
            default::STASHED_NERD,
        );
        let unpulled = handle_glyph(
            user.unpulled,
            user.ascii_symbols,
            default::UNPULLED,
            default::UNPULLED_NERD,
        );
        let unpushed = handle_glyph(
            user.unpushed,
            user.ascii_symbols,
            default::UNPUSHED,
            default::UNPUSHED_NERD,
        );
        let renamed = handle_glyph(
            user.renamed,
            user.ascii_symbols,
            default::RENAMED,
            default::RENAMED_NERD,
        );
        let staged = handle_glyph(
            user.staged,
            user.ascii_symbols,
            default::STAGED,
            default::STAGED_NERD,
        );
        let added = handle_glyph(
            user.added,
            user.ascii_symbols,
            default::ADDED,
            default::ADDED_NERD,
        );
        let staged_deleted = handle_glyph(
            user.staged_deleted,
            user.ascii_symbols,
            default::STAGED_DELETED,
            default::STAGED_DELETED_NERD,
        );
        let modified = handle_glyph(
            user.modified,
            user.ascii_symbols,
            default::MODIFIED,
            default::MODIFIED_NERD,
        );
        let unstaged = handle_glyph(
            user.unstaged,
            user.ascii_symbols,
            default::UNSTAGED,
            default::UNSTAGED_NERD,
        );
        let deleted = handle_glyph(
            user.deleted,
            user.ascii_symbols,
            default::DELETED,
            default::DELETED_NERD,
        );
        Self {
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
        }
    }
}

fn handle_glyph(
    glyph: Option<String>,
    ascii_symbols: bool,
    ascii_default: char,
    nerd_default: char,
) -> String {
    if let Some(glyph) = glyph {
        glyph
    } else if ascii_symbols {
        ascii_default.to_string()
    } else {
        nerd_default.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::super::input::UserGlyphs;
    use super::*;

    impl UserGlyphs {
        fn none(ascii_symbols: bool) -> Self {
            Self {
                ascii_symbols,
                stashed: None,
                unpulled: None,
                unpushed: None,
                renamed: None,
                staged: None,
                added: None,
                staged_deleted: None,
                modified: None,
                unstaged: None,
                deleted: None,
            }
        }
    }

    #[test]
    fn nerd() {
        let chosen = ChosenGlyphs::from(UserGlyphs::none(false));
        assert_eq!(default::STASHED_NERD.to_string(), chosen.stashed);
        assert_eq!(default::UNPULLED_NERD.to_string(), chosen.unpulled);
        assert_eq!(default::UNPUSHED_NERD.to_string(), chosen.unpushed);
        assert_eq!(default::RENAMED_NERD.to_string(), chosen.renamed);
        assert_eq!(default::STAGED_NERD.to_string(), chosen.staged);
        assert_eq!(default::ADDED_NERD.to_string(), chosen.added);
        assert_eq!(default::STAGED_DELETED_NERD.to_string(), chosen.staged_deleted);
        assert_eq!(default::MODIFIED_NERD.to_string(), chosen.modified);
        assert_eq!(default::UNSTAGED_NERD.to_string(), chosen.unstaged);
        assert_eq!(default::DELETED_NERD.to_string(), chosen.deleted);
    }

    #[test]
    fn ascii() {
        let chosen = ChosenGlyphs::from(UserGlyphs::none(true));
        assert_eq!(default::STASHED.to_string(), chosen.stashed);
        assert_eq!(default::UNPULLED.to_string(), chosen.unpulled);
        assert_eq!(default::UNPUSHED.to_string(), chosen.unpushed);
        assert_eq!(default::RENAMED.to_string(), chosen.renamed);
        assert_eq!(default::STAGED.to_string(), chosen.staged);
        assert_eq!(default::ADDED.to_string(), chosen.added);
        assert_eq!(default::STAGED_DELETED.to_string(), chosen.staged_deleted);
        assert_eq!(default::MODIFIED.to_string(), chosen.modified);
        assert_eq!(default::UNSTAGED.to_string(), chosen.unstaged);
        assert_eq!(default::DELETED.to_string(), chosen.deleted);
    }
}
