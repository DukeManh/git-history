use crate::git;
use serde::Serialize;

static SHOW: &str = "show";

#[derive(Serialize)]
pub struct Commit {
  sha: String,
  author: String,
  author_email: String,
  author_date: u32,
  commit_date: u32,
  parent_sha: String,
  message: String,
}


pub fn show(local_repo: &String, object: &String) -> Commit {
  // custom output format: "sha \n author_name \n author_email \n author_date \n commit_date \n parent_sha \n  message"
  let format = "--format=%H%n%aN%n%aE%n%at%n%ct%n%P%n%B";
  let args = [SHOW, &object, format, "--no-patch"];
  let out = git::cli::spawn(local_repo, args);

  let commit_info: Vec<_> = out.lines().collect();

   parse_commit(commit_info)
}

fn parse_commit(commit_info: Vec<&str>) -> Commit {
     Commit {
      sha: commit_info[0].to_string(),
      author: commit_info[1].to_string(),
      author_email: commit_info[2].to_string(),
      author_date: commit_info[3].parse::<u32>().unwrap(),
      commit_date: commit_info[4].parse::<u32>().unwrap(),
      parent_sha: commit_info[5].to_string(),
      message: commit_info[6].to_string(),
    }
}
