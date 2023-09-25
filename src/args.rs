use clap::Parser;

#[derive(Parser)]
#[command(author, about, version)]
pub struct Args {
	#[arg(short, long)]
	pub verbose: bool,
	#[arg(long)]
	pub ascii_symbols: bool,

	#[arg(long)]
	pub unpushed_color: Option<String>,
	#[arg(long)]
	pub all_staged_color: Option<String>,
	#[arg(long)]
	pub all_unstaged_color: Option<String>,
	#[arg(long)]
	pub renamed_color: Option<String>,
	#[arg(long)]
	pub added_color: Option<String>,
	#[arg(long)]
	pub staged_color: Option<String>,
	#[arg(long)]
	pub staged_deleted_color: Option<String>,
	#[arg(long)]
	pub modified_color: Option<String>,
	#[arg(long)]
	pub deleted_color: Option<String>,
	#[arg(long)]
	pub unstaged_color: Option<String>,

	#[arg(long)]
	pub unpushed_symbol: Option<String>,
	#[arg(long)]
	pub renamed_symbol: Option<String>,
	#[arg(long)]
	pub added_symbol: Option<String>,
	#[arg(long)]
	pub staged_symbol: Option<String>,
	#[arg(long)]
	pub staged_deleted_symbol: Option<String>,
	#[arg(long)]
	pub modified_symbol: Option<String>,
	#[arg(long)]
	pub deleted_symbol: Option<String>,
	#[arg(long)]
	pub unstaged_symbol: Option<String>,
}

pub struct UserColors {
	pub unpushed: Option<String>,
	pub all_staged: Option<String>,
	pub all_unstaged: Option<String>,
	pub unstaged: Option<String>,
	pub deleted: Option<String>,
	pub modified: Option<String>,
	pub added: Option<String>,
	pub staged: Option<String>,
	pub renamed: Option<String>,
	pub staged_deleted: Option<String>,
}

pub struct UserGlyphs {
	pub ascii_symbols: bool,
	pub unpushed: Option<String>,
	pub renamed: Option<String>,
	pub added: Option<String>,
	pub staged: Option<String>,
	pub staged_deleted: Option<String>,
	pub modified: Option<String>,
	pub deleted: Option<String>,
	pub unstaged: Option<String>,
}

impl UserGlyphs {
	pub fn none(ascii_symbols: bool) -> Self {
		Self {
			ascii_symbols,
			unpushed: None,
			renamed: None,
			added: None,
			staged: None,
			staged_deleted: None,
			modified: None,
			deleted: None,
			unstaged: None,
		}
	}
}
