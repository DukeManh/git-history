use crate::git;

static PARSE: &str = "rev-parse";

pub fn is_git_repo(local_repo: &String) -> String {
  let args = [PARSE, "--is-inside-work-tree"];

  return git::cli::spawn(local_repo, args);
}

pub fn get_branch(local_repo: &String) -> String {
  let args = [PARSE, "--abbrev-ref", "HEAD"];

  return git::cli::spawn(local_repo, args);
}
