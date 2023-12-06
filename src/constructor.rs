use self::colorizer::ChosenColors;
use self::glyphizer::ChosenGlyphs;
use self::input::UserColors;
use self::input::UserGlyphs;
use self::input::UserRemoves;
use crate::args::Args;
use crate::parser::Stats;
use ansi_term::ANSIString;
use ansi_term::Color;

mod colorizer;
mod glyphizer;
mod removizer;

mod input {
    pub struct UserColors {
        pub all_commits: Option<String>,
        pub all_staged: Option<String>,
        pub all_unstaged: Option<String>,
        pub stashed: Option<String>,
        pub unpulled: Option<String>,
        pub unpushed: Option<String>,
        pub renamed: Option<String>,
        pub staged: Option<String>,
        pub added: Option<String>,
        pub staged_deleted: Option<String>,
        pub modified: Option<String>,
        pub unstaged: Option<String>,
        pub deleted: Option<String>,
    }

    pub struct UserGlyphs {
        pub ascii_symbols: bool,
        pub stashed: Option<String>,
        pub unpulled: Option<String>,
        pub unpushed: Option<String>,
        pub renamed: Option<String>,
        pub staged: Option<String>,
        pub added: Option<String>,
        pub staged_deleted: Option<String>,
        pub modified: Option<String>,
        pub unstaged: Option<String>,
        pub deleted: Option<String>,
    }

    pub struct UserRemoves {
        pub stashed: bool,
        pub unpulled: bool,
        pub unpushed: bool,
        pub renamed: bool,
        pub staged: bool,
        pub added: bool,
        pub staged_deleted: bool,
        pub modified: bool,
        pub unstaged: bool,
        pub deleted: bool,
    }
}

pub fn construct(mut stat: Stats, args: Args) -> (Vec<ANSIString<'static>>, Vec<String>) {
    let mut elements: Vec<ANSIString<'static>> = vec![];

    stat.reset(UserRemoves {
        stashed: args.remove_stashed,
        unpulled: args.remove_unpulled,
        unpushed: args.remove_unpushed,
        renamed: args.remove_renamed,
        staged: args.remove_staged,
        added: args.remove_added,
        staged_deleted: args.remove_staged_deleted,
        modified: args.remove_modified,
        unstaged: args.remove_unstaged,
        deleted: args.remove_deleted,
    });

    let (colors, user_errors) = ChosenColors::from(UserColors {
        all_commits: args.color_all_commits,
        all_staged: args.color_all_staged,
        all_unstaged: args.color_all_unstaged,
        stashed: args.color_stashed,
        unpulled: args.color_unpulled,
        unpushed: args.color_unpushed,
        renamed: args.color_renamed,
        staged: args.color_staged,
        added: args.color_added,
        staged_deleted: args.color_staged_deleted,
        modified: args.color_modified,
        unstaged: args.color_unstaged,
        deleted: args.color_deleted,
    });

    let glyphs = ChosenGlyphs::from(UserGlyphs {
        ascii_symbols: args.ascii_symbols,
        stashed: args.symbol_stashed,
        unpulled: args.symbol_unpulled,
        unpushed: args.symbol_unpushed,
        renamed: args.symbol_renamed,
        staged: args.symbol_staged,
        added: args.symbol_added,
        staged_deleted: args.symbol_staged_deleted,
        modified: args.symbol_modified,
        unstaged: args.symbol_unstaged,
        deleted: args.symbol_deleted,
    });

    add_if_positive(&mut elements, colors.stashed, glyphs.stashed, stat.stashed);
    add_if_positive(&mut elements, colors.unpulled, glyphs.unpulled, stat.unpulled);
    add_if_positive(&mut elements, colors.unpushed, glyphs.unpushed, stat.unpushed);
    add_if_positive(&mut elements, colors.renamed, glyphs.renamed, stat.renamed);
    add_if_positive(&mut elements, colors.staged, glyphs.staged, stat.staged);
    add_if_positive(&mut elements, colors.added, glyphs.added, stat.added);
    add_if_positive(
        &mut elements,
        colors.staged_deleted,
        glyphs.staged_deleted,
        stat.staged_deleted,
    );
    add_if_positive(&mut elements, colors.modified, glyphs.modified, stat.modified);
    add_if_positive(&mut elements, colors.unstaged, glyphs.unstaged, stat.unstaged);
    add_if_positive(&mut elements, colors.deleted, glyphs.deleted, stat.deleted);
    (elements, user_errors)
}

fn add_if_positive(
    elements: &mut Vec<ANSIString<'static>>,
    color: Color,
    glyph: String,
    specific_stat: usize,
) {
    if specific_stat > 0 {
        elements.push(color.paint(format!("{}{} ", glyph, specific_stat)))
    }
}
