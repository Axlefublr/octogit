use std::process::Command;

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

pub fn remote() -> Result<String, String> {
	let output = match Command::new("git")
		.arg("remote")
		.output()
	{
		Err(_) => return Err("`git` is not in your $PATH".to_owned()),
		Ok(v) => v,
	};
	if !output.status.success() {
		return Err(String::from_utf8(output.stderr)
			.expect("git remote stderr should convert to a string")
			.trim()
			.to_owned());
	}
	let remote = String::from_utf8(output.stdout)
		.expect("git remote failed to convert to a string")
		.trim()
		.to_owned();
	if remote.is_empty() {
		return Err("this repository doesn't have a remote".to_owned());
	}
	let remote = remote.lines().next().expect("if remote isn't empty, it should be at least one line").to_owned();
	Ok(remote)
}

pub fn branch() -> Result<String, String> {
	let output = match Command::new("git")
		.arg("branch")
		.arg("--show-current")
		.output()
	{
		Err(_) => return Err("`git` is not in your $PATH".to_owned()),
		Ok(v) => v,
	};
	if !output.status.success() {
		return Err(String::from_utf8(output.stderr)
			.expect("git branch stderr should convert to a string")
			.trim()
			.to_owned());
	}
	let branch = String::from_utf8(output.stdout)
		.expect("git branch failed to convert to a string")
		.trim()
		.to_owned();
	if branch.is_empty() {
		return Err("this repository isn't on a branch".to_owned());
	} else {
		Ok(branch)
	}
}