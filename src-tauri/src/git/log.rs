use crate::git;

static LOG: &str = "log";

pub fn get_commits(local_repo: &String, args: &String) -> Vec<String> {
  let run = [LOG, " ", args].concat();

  let output = git::cli::spawn(&run, local_repo);
  let commits: Vec<_> = output.lines().collect();

  commits.into_iter().map(|s| String::from(s)).collect()
}
