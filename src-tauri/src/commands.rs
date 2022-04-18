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
      let is_git_repo = git::rev_parse::is_git_repo(&local_repo);

      if is_git_repo == "true" {
        let repo = String::from(path.file_name().unwrap().to_str().unwrap());
        let branch = git::rev_parse::get_branch(&local_repo);

        Ok((repo, branch))
      } else {
        Err("folder is not a git repository".into())
      }
    }
  }
}

#[command]
pub async fn get_commits(local_repo: String, limit: u16, before: String) -> Vec<String> {
  git::log::get_commits(&local_repo, &limit, &before)
}

use git::show::Commit;
use serde::Serialize;

#[derive(Serialize)]
pub struct Diff {
  commit: Commit,
  files: Vec<String>,
}

#[command]
pub async fn git_show(local_repo: String, object: String) -> Diff {
  let files = git::diff_tree::get_changed_files(&local_repo, &object);

  let commit = git::show::show(&local_repo, &object);

  Diff { commit, files }
}

#[command]
pub async fn git_show_diff(local_repo: String, object: String) -> String {
  git::diff::diff(&local_repo, &object)
}
