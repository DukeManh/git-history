use crate::git;

static PARSE: &str = "rev-parse";

pub fn is_git_repo(local_repo: &String) -> String {
  let run = [PARSE, " ", "--is-inside-work-tree"].concat();

  return git::cli::spawn(&run, local_repo);
}

pub fn get_branch(local_repo: &String) -> String {
  let run = [PARSE, " ", "--abbrev-ref HEAD"].concat();

  return git::cli::spawn(&run, local_repo);
}
