use std::fs;
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
    arg_input_file: Option<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    match fs::read_to_string(args.arg_input_file.unwrap()) {
      Ok(input_file_text) => {
        let commands = proj_struct::parse(input_file_text);
        if args.flag_pretend {
          for cmd in commands {
              println!("would have executed: {}", cmd);
          }
        } else {
          for cmd in commands {
              cmd.execute();
          }
        }
      },
      Err(e) => eprintln!("{}", e)
    }
}
