use crate::git;

static COMMAND: &str = "rev-parse";

pub fn is_git_repo(local_repo: &String) -> String {
  let run = String::from(COMMAND) + &String::from(" --is-inside-work-tree");
  return git::cli::spawn(&run, local_repo);
}

pub fn get_branch(local_repo: &String) -> String {
  let run = String::from(COMMAND) + &String::from(" --abbrev-ref HEAD");
  return git::cli::spawn(&run, local_repo);
}
