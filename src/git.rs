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
	Ok(git_status)
}

fn remote() -> Result<String, String> {
	let output = match Command::new("git").arg("remote").output() {
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
	let remote = remote
		.lines()
		.next()
		.expect("if remote isn't empty, it should be at least one line")
		.to_owned();
	Ok(remote)
}

fn branch() -> Result<String, String> {
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
		Err("this repository isn't on a branch".to_owned())
	} else {
		Ok(branch)
	}
}

fn log(remote: &str, branch: &str) -> Result<String, String> {
	let output = match Command::new("git")
		.arg("log")
		.arg("--oneline")
		.arg(format!("{0}/{1}..{1}", remote, branch))
		.output()
	{
		Err(_) => return Err("`git` is not in your $PATH".to_owned()),
		Ok(v) => v,
	};
	if !output.status.success() {
		return Err(String::from_utf8(output.stderr)
			.expect("git log stderr should convert to a string")
			.trim()
			.to_owned());
	}
	let log = String::from_utf8(output.stdout)
		.expect("git log failed to convert to a string")
		.trim()
		.to_owned();
	Ok(log)
}
pub fn get_unpushed() -> Result<usize, String> {
	let remote = remote()?;
	let branch = branch()?;
	let log = log(&remote, &branch)?;
	let commits: usize = log.lines().count();
	Ok(commits)
}
