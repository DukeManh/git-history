use crate::git;

static COMMAND: &str = "rev-parse";

pub fn is_git_repo(local_repo: &String) -> String {
  let run = [COMMAND, " ", "--is-inside-work-tree"].concat();

  return git::cli::spawn(&run, local_repo);
}

pub fn get_branch(local_repo: &String) -> String {
  let run = [COMMAND, " ", "--abbrev-ref HEAD"].concat();

  return git::cli::spawn(&run, local_repo);
}
