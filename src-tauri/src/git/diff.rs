pub fn diff_commits(sha1: String, sha2: String) {
  println!("git diff commits: {} vs {}", sha1, sha2);
}

pub fn diff_tags(tags1: String, tags2: String) {
  println!("git diff tags: {} vs {}", tags1, tags2);
}
