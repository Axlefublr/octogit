use crate::git::Commits;

#[derive(Debug, Default)]
pub struct Stats {
	pub stashed: usize,
	pub unpulled: usize,
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
	pub fn compute(git_status: String, commits: Commits, stashes: usize) -> Option<Self> {
		let mut stats = Stats::default();
		if !git_status.is_empty() {
			parse_status(git_status, &mut stats)?;
		}
		stats.stashed = stashes;
		stats.unpulled = commits.unpulled;
		stats.unpushed = commits.unpushed;
		if are_all_zero(&stats) {
			None
		} else {
			Some(stats)
		}
	}

	pub fn one() -> Self {
		Self {
			stashed: 1,
			unpulled: 1,
			unpushed: 1,
			unstaged: 1,
			added: 1,
			staged: 1,
			modified: 1,
			renamed: 1,
			deleted: 1,
			staged_deleted: 1,
		}
	}
}

fn parse_status(git_status: String, stats: &mut Stats) -> Option<()> {
	for line in git_status.lines() {
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
	stats.stashed == 0
		&& stats.unpulled == 0
		&& stats.unpushed == 0
		&& stats.renamed == 0
		&& stats.staged == 0
		&& stats.added == 0
		&& stats.staged_deleted == 0
		&& stats.modified == 0
		&& stats.unstaged == 0
		&& stats.deleted == 0
}
