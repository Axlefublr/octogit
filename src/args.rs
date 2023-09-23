use clap::Parser;

#[derive(Parser)]
#[command(author, about, version)]
pub struct Args {
	#[arg(short, long)]
	pub verbose: bool
}