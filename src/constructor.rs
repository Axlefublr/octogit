use ansi_term::ANSIString;
use ansi_term::Color;

use crate::args::Args;
use crate::parser::Stats;

use self::colorizer::ChosenColors;

mod colorizer;
mod glyphizer;

pub fn construct(stat: Stats, args: Args) -> (Vec<ANSIString<'static>>, Vec<String>) {
	let mut elements: Vec<ANSIString<'static>> = vec![];
	let (colors, user_errors) = ChosenColors::from(
		args.unpushed,
		args.all_staged,
		args.all_unstaged,
		args.unstaged,
		args.deleted,
		args.modified,
		args.added,
		args.staged,
		args.renamed,
		args.staged_deleted,
	);
	add_if_positive(&mut elements, colors.unpushed, '?', stat.unpushed);
	add_if_positive(&mut elements, colors.renamed, '?', stat.renamed);
	add_if_positive(&mut elements, colors.added, '?', stat.added);
	add_if_positive(&mut elements, colors.staged_deleted, '?', stat.staged_deleted);
	add_if_positive(&mut elements, colors.staged, '?', stat.staged);
	add_if_positive(&mut elements, colors.modified, '?', stat.modified);
	add_if_positive(&mut elements, colors.deleted, '?', stat.deleted);
	add_if_positive(&mut elements, colors.unstaged, '?', stat.unstaged);
	(elements, user_errors)
}

fn add_if_positive(
	elements: &mut Vec<ANSIString<'static>>,
	color: Color,
	glyph: char,
	specific_stat: usize,
) {
	if specific_stat > 0 {
		elements.push(color.paint(format!("{}{} ", glyph, specific_stat)))
	}
}
