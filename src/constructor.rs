use self::colorizer::ChosenColors;
use self::glyphizer::ChosenGlyphs;
use crate::args::Args;
use crate::args::UserColors;
use crate::args::UserGlyphs;
use crate::parser::Stats;
use ansi_term::ANSIString;
use ansi_term::Color;

mod colorizer;
pub mod glyphizer;

pub fn construct(stat: Stats, args: Args) -> (Vec<ANSIString<'static>>, Vec<String>) {
	let mut elements: Vec<ANSIString<'static>> = vec![];
	let (colors, user_errors) = ChosenColors::from(UserColors {
		unpushed: args.unpushed_color,
		all_staged: args.all_staged_color,
		all_unstaged: args.all_unstaged_color,
		renamed: args.renamed_color,
		added: args.added_color,
		staged: args.staged_color,
		staged_deleted: args.staged_deleted_color,
		modified: args.modified_color,
		deleted: args.deleted_color,
		unstaged: args.unstaged_color,
	});
	let glyphs = ChosenGlyphs::from(UserGlyphs {
		ascii_symbols: args.ascii_symbols,
		unpushed: args.unpushed_symbol,
		renamed: args.renamed_symbol,
		added: args.added_symbol,
		staged: args.staged_symbol,
		staged_deleted: args.staged_deleted_symbol,
		modified: args.modified_symbol,
		deleted: args.deleted_symbol,
		unstaged: args.unstaged_symbol,
	});
	add_if_positive(
		&mut elements,
		colors.unpushed,
		glyphs.unpushed,
		stat.unpushed,
	);
	add_if_positive(&mut elements, colors.renamed, glyphs.renamed, stat.renamed);
	add_if_positive(&mut elements, colors.added, glyphs.added, stat.added);
	add_if_positive(
		&mut elements,
		colors.staged_deleted,
		glyphs.staged_deleted,
		stat.staged_deleted,
	);
	add_if_positive(&mut elements, colors.staged, glyphs.staged, stat.staged);
	add_if_positive(
		&mut elements,
		colors.modified,
		glyphs.modified,
		stat.modified,
	);
	add_if_positive(&mut elements, colors.deleted, glyphs.deleted, stat.deleted);
	add_if_positive(
		&mut elements,
		colors.unstaged,
		glyphs.unstaged,
		stat.unstaged,
	);
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
