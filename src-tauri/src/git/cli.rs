use std::process::Command;

pub fn spawn(command: &String, working_directory: &String) -> String {
  let output = Command::new("git")
    .current_dir(working_directory)
    .args(command.split(" "))
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
