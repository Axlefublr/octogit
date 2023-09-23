﻿use crate::args::Args;

#[derive(Debug)]
pub struct Stats {
	pub unpushed: usize,
	pub unstaged: usize,
	pub added: usize,
	pub staged: usize,
	pub modified: usize,
	pub renamed: usize,
	pub deleted: usize,
	pub staged_deleted: usize,
}

impl Stats {
	fn new() -> Self {
		Self {
			unstaged: 0,
			added: 0,
			staged: 0,
			modified: 0,
			renamed: 0,
			deleted: 0,
			unpushed: 0,
			staged_deleted: 0
		}
	}
	pub fn from(args: Args) -> Option<Self> {
		let mut stats = Stats::new();
		if args.status.is_some() {
			parse_status(&args, &mut stats)?;
		}
		stats.unpushed = args.unpushed;
		if are_all_zero(&stats) {
			None
		} else {
			Some(stats)
		}
	}
}

fn parse_status(args: &Args, stats: &mut Stats) -> Option<()> {
	for line in args.status.as_ref().unwrap().lines() {
		let mut chars = line.chars();
		let first = chars.next()?;
		match first {
			'M' => stats.staged += 1,
			'A' => stats.added += 1,
			'D' => stats.staged_deleted += 1,
			'R' => stats.renamed += 1,
			'?' => {
				stats.unstaged += 1;
				continue;
			}
			_ => (),
		}
		let second = chars.next()?;
		match second {
			'M' => stats.modified += 1,
			'D' => stats.deleted += 1,
			_ => (),
		}
	}
	Some(())
}

fn are_all_zero(stats: &Stats) -> bool {
	stats.added == 0
		&& stats.deleted == 0
		&& stats.modified == 0
		&& stats.renamed == 0
		&& stats.staged == 0
		&& stats.unpushed == 0
		&& stats.unstaged == 0
		&& stats.staged_deleted == 0
}