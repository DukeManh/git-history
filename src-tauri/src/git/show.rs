use crate::git;
use serde::ser::{Serialize, SerializeStruct, Serializer};

static SHOW: &str = "show";

pub struct Commit {
  sha: String,
  author: String,
  author_email: String,
  author_date: u32,
  commit_date: u32,
  parent_sha: String,
  message: String,
}

impl Serialize for Commit {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut s: <S as Serializer>::SerializeStruct = serializer.serialize_struct("Commit", 6)?;
    s.serialize_field("sha", &self.sha)?;
    s.serialize_field("author", &self.author)?;
    s.serialize_field("author_email", &self.author_email)?;
    s.serialize_field("author_date", &self.author_date)?;
    s.serialize_field("commit_date", &self.commit_date)?;
    s.serialize_field("parent_sha", &self.parent_sha)?;
    s.serialize_field("message", &self.message)?;
    s.end()
  }
}

pub fn show(local_repo: &String, args: &String) -> Result<Commit, String> {
  // custom output format: "sha \n author_name \n author_email \n author_date \n commit_date \n parent_sha \n  message"
  let format = "--format=%H%n%aN%n%aE%n%at%n%ct%n%P%n%B";
  let run = [SHOW, " ", args, " ", format, " --no-patch"].concat();
  let out = git::cli::spawn(&run, local_repo);

  let commit_info: Vec<_> = out.lines().collect();

  if commit_info.len() < 7 {
    Err(String::from("Invalid commit object"))
  } else {
    let commit = parse_commit(commit_info);
    Ok(commit)
  }
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
