use std::process::ExitCode;

use clap::Parser;
use ansi_term::ANSIStrings;
use args::Args as Args;
use parser::Stats as Stats;
use colorizer::colorize;

mod args;
mod parser;
mod colorizer;
mod git;

fn main() {
	let args = Args::parse();
	let git_status = match git::status() {
		Ok(status) => status,
		Err(message) => {
			if args.verbose {
				eprintln!("{}", message);
			}
			String::from("")
		}
	};
	let unpushed = match git::get_unpushed() {
		Ok(unpushed) => unpushed,
		Err(message) => {
			if args.verbose {
				eprintln!("{}", message);
			}
			0
		}
	};
	let stats = Stats::compute(git_status, unpushed);
	if let Some(stats) = stats {
		let elements = colorize(stats);
		print!("{}", ANSIStrings(&elements));
	}
}