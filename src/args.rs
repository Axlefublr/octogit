use clap::Parser;

#[derive(Parser)]
#[command(author, about, version)]
pub struct Args {
	#[arg(short, long)]
	pub verbose: bool,
	#[arg(long)]
	pub unpushed: Option<String>,
	#[arg(long)]
	pub all_staged: Option<String>,
	#[arg(long)]
	pub all_unstaged: Option<String>,
	#[arg(long)]
	pub unstaged: Option<String>,
	#[arg(long)]
	pub deleted: Option<String>,
	#[arg(long)]
	pub modified: Option<String>,
	#[arg(long)]
	pub added: Option<String>,
	#[arg(long)]
	pub staged: Option<String>,
	#[arg(long)]
	pub renamed: Option<String>,
	#[arg(long)]
	pub staged_deleted: Option<String>
}
