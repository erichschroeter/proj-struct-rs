use docopt::Docopt;
use serde::Deserialize;

mod proj_struct;

const USAGE: &'static str = "
proj-struct

Usage:
  proj-struct [--pretend] <input-file>

Options:
  --pretend  Only pretend to execute commands.

<input-file> Rules:
  1. Directories are lines that end with a path separator '/'.
  2. Files are lines that do not end with a path separator '/'.
  3. Empty lines are ignored.

<input-file> Example:
  /a/path/to/a/directory/
  another/path/to/a/directory/
  a.txt
  a/path/to/b.txt
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_pretend: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
    let commands = proj_struct::parse("/a/path/with/multiple/directories/".to_string());
    for cmd in commands {
        println!("would have executed: {}", cmd.args[0]);
    }
}
