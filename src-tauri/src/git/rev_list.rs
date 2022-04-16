use crate::git::cli;

pub fn is_first_commit(local_repo: &String, commit: &String) -> bool {
  let run = ["rev-list", " -2 ", commit].concat();
  let output = cli::spawn(&run, &local_repo);
  let num_of_commits = output.trim().lines(); 

  if num_of_commits.count() < 2 {
    true
  } else {
    false
  }
}