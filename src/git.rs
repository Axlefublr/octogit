use std::process::Command;

fn get_repo_root() -> Result<String, String> {
	let output = match Command::new("git")
		.arg("rev-parse")
		.arg("--show-toplevel")
		.output()
	{
		Err(_) => return Err("`git` is not in your $PATH".to_owned()),
		Ok(v) => v,
	};
	if !output.status.success() {
		return Err(String::from_utf8(output.stderr)
			.expect("git rev-parse stderr should convert to a string")
			.trim()
			.to_owned());
	}
	let root_dir = String::from_utf8(output.stdout)
		.expect("git rev-parse failed to convert to a string")
		.trim()
		.to_owned();
	Ok(root_dir)
}

pub fn status() -> Result<String, String> {
	let output = match Command::new("git")
		.arg("status")
		.arg("--porcelain")
		.output()
	{
		Err(_) => return Err("`git` is not in your $PATH".to_owned()),
		Ok(v) => v,
	};
	if !output.status.success() {
		return Err(String::from_utf8(output.stderr)
			.expect("git status stderr should convert to a string")
			.trim()
			.to_owned());
	}
	let git_status = String::from_utf8(output.stdout)
		.expect("git status --porcelain failed to convert to a string")
		.trim_end()
		.to_owned();
	if git_status.is_empty() {
		Err("there are no git changes in this directory".to_owned())
	} else {
		Ok(git_status)
	}
}
