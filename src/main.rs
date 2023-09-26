use ansi_term::ANSIStrings;
use args::Args;
use clap::Parser;
use parser::Stats;

mod args;
mod constructor;
mod git;
mod parser;

fn main() {
	let args = Args::parse();
	let verbose = args.verbose;
	let git_status = match git::status() {
		Ok(status) => status,
		Err(message) => {
			if verbose {
				eprintln!("{}", message);
			}
			String::from("")
		}
	};
	let unpushed = match git::get_unpushed() {
		Ok(unpushed) => unpushed,
		Err(message) => {
			if verbose {
				eprintln!("{}", message);
			}
			0
		}
	};
	let stats = Stats::compute(git_status, unpushed);
	if let Some(stats) = stats {
		let (elements, user_errors) = constructor::construct(stats, args);
		if !user_errors.is_empty() && verbose {
			eprintln!("{}", user_errors.join("\n"));
		}
		print!("{}", ANSIStrings(&elements));
	}
}
