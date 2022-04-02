use crate::git;

pub fn get_commits(local_repo: &String, limit: &u16) -> Vec<String> {
  let output = git::cli::spawn(
    &String::from(format!("log -{} --oneline", limit)),
    local_repo,
  );

  let commits: Vec<_> = output.split("\n").collect();

  commits.into_iter().map(|s| String::from(s)).collect()
}
