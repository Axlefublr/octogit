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
	/// Will print every single element.
	///
	/// Helpful for testing the output without having to be in a git directory with certain changes.
	#[arg(long)]
	pub test: bool,

	/// [default: yellow]
	///
	/// For every color, you can either specify one of the main 8 colors from your terminal color scheme:
	///
	/// black, red, green, yellow, blue, purple, cyan, white
	///
	/// Or a hex code, without the # symbol like: FFAFD7
	///
	/// For both hex codes and color names, the cAsE doesn't matter
	#[arg(long)]
	pub color_unpushed: Option<String>,
	/// [default: green]
	#[arg(long)]
	pub color_all_staged: Option<String>,
	/// [default: cyan]
	#[arg(long)]
	pub color_all_unstaged: Option<String>,
	/// [default: --color-all-staged]
	#[arg(long)]
	pub color_renamed: Option<String>,
	/// [default: --color-all-staged]
	#[arg(long)]
	pub color_staged: Option<String>,
	/// [default: --color-all-staged]
	#[arg(long)]
	pub color_added: Option<String>,
	/// [default: --color-all-staged]
	#[arg(long)]
	pub color_staged_deleted: Option<String>,
	/// [default: --color-all-unstaged]
	#[arg(long)]
	pub color_modified: Option<String>,
	/// [default: --color-all-unstaged]
	#[arg(long)]
	pub color_unstaged: Option<String>,
	/// [default: --color-all-unstaged]
	#[arg(long)]
	pub color_deleted: Option<String>,

	/// [default:  or >]
	#[arg(long)]
	pub symbol_unpushed: Option<String>,
	/// [default: 󰕍 or &]
	#[arg(long)]
	pub symbol_renamed: Option<String>,
	/// [default: 󰄬 or !]
	#[arg(long)]
	pub symbol_staged: Option<String>,
	/// [default: 󰐕 or +]
	#[arg(long)]
	pub symbol_added: Option<String>,
	/// [default: 󰍴 or -]
	#[arg(long)]
	pub symbol_staged_deleted: Option<String>,
	/// [default:  or !]
	#[arg(long)]
	pub symbol_modified: Option<String>,
	/// [default: 󰐕 or +]
	#[arg(long)]
	pub symbol_unstaged: Option<String>,
	/// [default: 󰍴 or -]
	#[arg(long)]
	pub symbol_deleted: Option<String>,
}
