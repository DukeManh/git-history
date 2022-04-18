use crate::git;

static LOG: &str = "log";

pub fn get_commits(local_repo: &String, limit: &u16, before: &String) -> Vec<String> {
  let args =[LOG, &format!("-{}", limit), "--oneline", &before];

  let output = git::cli::spawn( local_repo, args);
  let commits: Vec<_> = output.lines().collect();

  commits.into_iter().map(|s| String::from(s)).collect()
}
