use std::fs::{File, create_dir_all};
use std::path::Path;

pub trait Command {
    fn execute(&self);
    fn to_string(&self) -> String;
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug)]
pub struct MkdirCommand {
    pub path: String,
}

impl MkdirCommand {
    pub fn new(path_str: &str) -> Self {
        let as_path = Path::new(path_str);
        match as_path.strip_prefix("/") {
            Ok(relative_path) => match relative_path.to_str() {
                Some(r) => MkdirCommand {
                    path: String::from(r),
                },
                None => MkdirCommand {
                    path: String::from(path_str),
                },
            },
            Err(_) => MkdirCommand {
                path: String::from(path_str),
            },
        }
    }
}

impl Command for MkdirCommand {
    fn execute(&self) {
        match create_dir_all(Path::new(&self.path)) {
            Ok(_) => (),
            Err(_) => (),
        }
    }

    fn to_string(&self) -> String {
        format!("mkdir -p {}", self.path)
    }
}

#[derive(Debug)]
pub struct TouchCommand {
    pub path: String,
    pub contents: String,
}

impl Command for TouchCommand {
    fn execute(&self) {
        match File::create(&self.path) {
            Ok(_) => (),
            Err(_) => (),
        }
    }

    fn to_string(&self) -> String {
        format!("touch {}", self.path)
    }
}

pub fn is_dir(path: &str) -> bool {
    if path.len() > 0 {
        if path.chars().last().unwrap() == '/' {
            return true;
        }
    }
    false
}

pub fn parse(input_string: String) -> Vec<Box<dyn Command>> {
    let mut output: Vec<Box<dyn Command>> = vec![];
    let lines = input_string.split("\n");

    for line in lines {
        if is_dir(line) {
            output.push(Box::new(MkdirCommand::new(line)));
        } else if !line.is_empty() {
            let folder = Path::new(line);

            match folder.parent() {
                Some(base) => {
                    let base_str = base.to_str().unwrap();
                    if !base_str.is_empty() {
                        output.push(Box::new(MkdirCommand::new(base_str)))
                    }
                }
                None => (),
            }

            output.push(Box::new(TouchCommand {
                path: String::from(line),
                contents: String::from(""),
            }));
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let commands = parse("/a/path/with/multiple/directories/".to_string());
        assert_eq!(
            &commands[0].to_string(),
            "mkdir -p a/path/with/multiple/directories"
        );
    }

    #[test]
    fn test_is_dir_returns_false_for_invalid_directory() {
        assert!(!is_dir("example/file"));
    }

    #[test]
    fn test_is_dir_returns_true_for_valid_directory() {
        assert!(is_dir("example/folder/"));
    }

    #[test]
    fn test_parsing_empty_lines_between_two_non_empty_lines_is_ignored() {
        let commands = parse(String::from(
            "/a/valid/directory/before/empty/line/

/a/valid/directory/after/empty/line/",
        ));
        assert_eq!(commands.len(), 2);
    }

    #[test]
    fn test_parsing_absolute_folder_paths_are_converted_to_relative_paths() {
        let commands = parse(String::from("/an/absolute/directory/"));
        assert_eq!(commands[0].to_string(), "mkdir -p an/absolute/directory");
    }
}
