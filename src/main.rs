use clap::Parser;
use ansi_term::ANSIStrings;
use args::Args as Args;
use parser::Stats as Stats;
use colorizer::colorize;

mod args;
mod parser;
mod colorizer;

fn main() {
	let args = Args::parse();
	let stats = Stats::from(args);
	if let Some(stats) = stats {
		let elements = colorize(stats);
		print!("{}", ANSIStrings(&elements));
	}
}