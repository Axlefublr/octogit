use ansi_term::ANSIString;
use ansi_term::Color;

use crate::parser::Stats;

mod default_colors;
mod glyphs;

pub fn colorize(stat: Stats) -> Vec<ANSIString<'static>> {
	let mut elements: Vec<ANSIString<'static>> = vec![];
	add_if_positive(&mut elements, default_colors::YELLOW, glyphs::UNPUSHED, stat.unpushed);
	add_if_positive(&mut elements, default_colors::GREEN, glyphs::RENAMED, stat.renamed);
	add_if_positive(&mut elements, default_colors::GREEN, glyphs::ADDED, stat.added);
	add_if_positive(&mut elements, default_colors::GREEN, glyphs::DELETED, stat.staged_deleted);
	add_if_positive(&mut elements, default_colors::GREEN, glyphs::STAGED, stat.staged);
	add_if_positive(&mut elements, default_colors::CYAN, glyphs::MODIFIED, stat.modified);
	add_if_positive(&mut elements, default_colors::CYAN, glyphs::DELETED, stat.deleted);
	add_if_positive(&mut elements, default_colors::CYAN, glyphs::UNSTAGED, stat.unstaged);
	elements
}

fn add_if_positive(elements: &mut Vec<ANSIString<'static>>, color: Color, glyph: char, specific_stat: usize) {
	if specific_stat > 0 {
		elements.push(color.paint(format!("{}{} ", glyph, specific_stat)))
	}
}