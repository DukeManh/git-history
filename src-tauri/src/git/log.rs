use crate::git;

static LOG: &str = "log";

pub fn get_commits(local_repo: &String, args: &String) -> Vec<String> {
  let args = [LOG, " ", args];

  let output = git::cli::spawn( local_repo, args);
  let commits: Vec<_> = output.lines().collect();

  commits.into_iter().map(|s| String::from(s)).collect()
}
