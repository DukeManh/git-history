use std::fs;
use std::path::Path;
use tauri::command;

use crate::git;

#[command]
pub fn read_repo(local_repo: String) -> Result<(String, String), String> {
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
