use std::{process::Command, ffi::OsStr};

pub fn spawn<I, S>(working_directory: &String, args: I) -> String
  where
      I: IntoIterator<Item = S>,
      S: AsRef<OsStr>,
 {
    let output = Command::new("git")
    .current_dir(working_directory)
    .args(["--no-pager"])
    .args(args)
    .output()
    .unwrap_or_else(|e| panic!("Failed to run Git command: {}", e));

  if output.status.success() {
    let stdout = String::from_utf8(output.stdout);

    if stdout.is_ok() {
      String::from(stdout.unwrap().trim())
    } else {
      String::from("")
    }
  } else {
    let s = String::from_utf8_lossy(&output.stderr);

    println!("Git Error: {}\n", s);

    String::from("")
  }
}
