use clap::Parser;

#[derive(Parser)]
#[command(author, about, long_about = None)]
#[command(next_line_help = true)]
pub struct Args {
	#[arg(long)]
	pub status: Option<String>,
	#[arg(long)]
	pub unpushed: usize
}