use crate::git;

pub fn diff(local_repo: &String, object: &String ) -> String {
  
  let is_first_commit = git::rev_list::is_first_commit(&local_repo, object);

  let previous_commit = if is_first_commit {
    let empty_repo_id = String::from("4b825dc642cb6eb9a060e54bf8d69288fbee4904");

    empty_repo_id   
  } else {
    [object, "~1"].concat()
  };

  let run: String = ["diff ", object, " ", &previous_commit].concat();
  let out = git::cli::spawn(&run, &local_repo);

  out
}