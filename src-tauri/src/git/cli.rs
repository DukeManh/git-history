use std::{ffi::OsStr, process::Command};

pub fn spawn<I, S>(working_directory: &String, args: I) -> String
where
  I: IntoIterator<Item = S>,
  S: AsRef<OsStr>,
{
  let mut command = Command::new("git");

  command
    .current_dir(working_directory)
    .args(["--no-pager"])
    .args(args);

  let output = command
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

    /* Use for debugging
    let arguments: Vec<_> = command.get_args().collect();
    println!("{:?}\n", arguments);
    */
    println!("Git Command finished with errors: {}\n", s);

    String::from("")
  }
}
