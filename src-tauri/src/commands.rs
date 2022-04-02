use std::fs;
use std::path::Path;
use tauri::command;

use crate::git;

#[command]
pub async fn read_repo(local_repo: String) -> Result<(String, String), String> {
  let path = Path::new(&local_repo);

  match fs::read_dir(&path) {
    Err(_why) => Err("path is not a directory".into()),
    Ok(_files) => {
      let is_git_repo = git::revision::is_git_repo(&local_repo);

      if is_git_repo == "true" {
        let repo = String::from(path.file_name().unwrap().to_str().unwrap());
        let branch = git::revision::get_branch(&local_repo);

        Ok((repo, branch))
      } else {
        Err("folder is not a git repository".into())
      }
    }
  }
}

#[command]
pub async fn get_commits(local_repo: String, limit: u16, before: String) -> Vec<String> {
  let args = [&format!("-{} --oneline", limit), " ", &before].concat();

  let commits = git::log::get_commits(&local_repo, &args);

  commits
}
