use ansi_term::ANSIStrings;
use args::Args;
use clap::Parser;
use git::Commits;
use parser::Stats;

mod args;
mod constructor;
mod git;
mod parser;

fn main() {
    let args = Args::parse();
    let verbose = args.verbose;
    let git_status = git::status().unwrap_or_else(|message| {
        if verbose {
            eprintln!("{}", message);
        }
        String::default()
    });
    let commits = git::get_commits().unwrap_or_else(|message| {
        if verbose {
            eprintln!("{}", message);
        }
        Commits::default()
    });
    let stashes = git::stashes().unwrap_or_else(|message| {
        if verbose {
            eprintln!("{}", message);
        }
        usize::default()
    });
    let stats = if args.test {
        Some(Stats::one())
    } else {
        Stats::compute(git_status, commits, stashes)
    };
    if let Some(stats) = stats {
        let (elements, user_errors) = constructor::construct(stats, args);
        if !user_errors.is_empty() && verbose {
            eprintln!("{}", user_errors.join("\n"));
        }
        print!("{}", ANSIStrings(&elements));
    }
}
