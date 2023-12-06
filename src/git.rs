use std::process::Command;

pub fn status() -> Result<String, String> {
    let output = match Command::new("git").arg("status").arg("--porcelain").output() {
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
    let output = match Command::new("git").arg("branch").arg("--show-current").output() {
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

pub fn stashes() -> Result<usize, String> {
    let output = match Command::new("git").arg("stash").arg("list").output() {
        Err(_) => return Err("`git` is not in your $PATH".to_owned()),
        Ok(v) => v,
    };
    if !output.status.success() {
        return Err(String::from_utf8(output.stderr)
            .expect("git stash stderr failed to convert to a string")
            .trim()
            .to_owned());
    }
    let stashes: usize = String::from_utf8(output.stdout)
        .expect("git stash failed to convert to a string")
        .trim()
        .lines()
        .count();
    Ok(stashes)
}

fn rev_list(range: &str) -> Result<usize, String> {
    let output = match Command::new("git")
        .arg("rev-list")
        .arg("--count")
        .arg(range)
        .output()
    {
        Err(_) => return Err("`git` is not in your $PATH".to_owned()),
        Ok(v) => v,
    };
    if !output.status.success() {
        return Err(String::from_utf8(output.stderr)
            .expect("git rev-list stderr failed to convert to a string")
            .trim()
            .to_owned());
    }
    let commits: usize = String::from_utf8(output.stdout)
        .expect("git rev-list failed to convert to a string")
        .trim()
        .parse()
        .expect("git rev-list failed to parse into an int");
    Ok(commits)
}

#[derive(Default)]
pub struct Commits {
    pub unpulled: usize,
    pub unpushed: usize,
}

pub fn get_commits() -> Result<Commits, String> {
    let remote = remote()?;
    let branch = branch()?;
    let unpulled = rev_list(&format!("{0}..{1}/{0}", &branch, &remote))?;
    let unpushed = rev_list(&format!("{1}/{0}..{0}", &branch, &remote))?;
    Ok(Commits { unpulled, unpushed })
}
