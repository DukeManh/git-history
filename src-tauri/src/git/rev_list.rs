use crate::git::cli;

pub fn is_first_commit(local_repo: &String, commit: &String) -> bool {
  let args = ["rev-list", " -2 ", commit];
  let output = cli::spawn(&local_repo, args );
  let num_of_commits = output.trim().lines(); 

  if num_of_commits.count() < 2 {
    true
  } else {
    false
  }
}