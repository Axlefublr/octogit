use clap::Parser;

#[derive(Parser)]
#[command(author, about, version)]
pub struct Args {
	/// octussy doesn't print errors by default, because it's supposed to be in your shell prompt constantly.
	///
	/// When you do want to see the errors, use this flag.
	#[arg(short, long)]
	pub verbose: bool,
	/// octussy uses nerd font symbols for some elements by default.
	///
	/// Use this flag if you don't use a nerd font.
	///
	/// You can see both the nerd and ascii defaults for every category later down in this help.
	#[arg(long)]
	pub ascii_symbols: bool,

	/// [default: yellow]
	#[arg(long)]
	pub unpushed_color: Option<String>,
	/// [default: green]
	#[arg(long)]
	pub all_staged_color: Option<String>,
	/// [default: cyan]
	#[arg(long)]
	pub all_unstaged_color: Option<String>,
	/// [default: all_staged_color]
	#[arg(long)]
	pub renamed_color: Option<String>,
	/// [default: all_staged_color]
	#[arg(long)]
	pub added_color: Option<String>,
	/// [default: all_staged_color]
	#[arg(long)]
	pub staged_color: Option<String>,
	/// [default: all_staged_color]
	#[arg(long)]
	pub staged_deleted_color: Option<String>,
	/// [default: all_unstaged_color]
	#[arg(long)]
	pub modified_color: Option<String>,
	/// [default: all_unstaged_color]
	#[arg(long)]
	pub deleted_color: Option<String>,
	/// [default: all_unstaged_color]
	#[arg(long)]
	pub unstaged_color: Option<String>,

	/// [default:  or >]
	#[arg(long)]
	pub unpushed_symbol: Option<String>,
	/// [default: 󰹹 or &]
	#[arg(long)]
	pub renamed_symbol: Option<String>,
	/// [default: +]
	#[arg(long)]
	pub added_symbol: Option<String>,
	/// [default: 󰄬 or *]
	#[arg(long)]
	pub staged_symbol: Option<String>,
	/// [default:  or ×]
	#[arg(long)]
	pub staged_deleted_symbol: Option<String>,
	/// [default: !]
	#[arg(long)]
	pub modified_symbol: Option<String>,
	/// [default:  or ×]
	#[arg(long)]
	pub deleted_symbol: Option<String>,
	/// [default: ?]
	#[arg(long)]
	pub unstaged_symbol: Option<String>,
}

pub struct UserColors {
	pub unpushed: Option<String>,
	pub all_staged: Option<String>,
	pub all_unstaged: Option<String>,
	pub renamed: Option<String>,
	pub added: Option<String>,
	pub staged: Option<String>,
	pub staged_deleted: Option<String>,
	pub modified: Option<String>,
	pub deleted: Option<String>,
	pub unstaged: Option<String>,
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
	#[allow(dead_code)] // because it *is* actually used in crate::glyphizer::tests, but isn't recognized as being used for some reason
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
