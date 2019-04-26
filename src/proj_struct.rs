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

impl Command for MkdirCommand {
    fn execute(&self) {
        println!("mkdir -p {}", self.path);
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
        println!("touch {}", self.path);
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
            output.push(Box::new(
                MkdirCommand {
                    path: String::from(line)
                }
            ));
        } else {
            let folder = Path::new(line);

            match folder.parent() {
                Some(base) => {
                    let base_str = base.to_str().unwrap();
                    if !base_str.is_empty() {
                        output.push(Box::new(
                            MkdirCommand {
                                path: String::from(base_str)
                            }
                        ))
                    }
                },
                None => ()
            }

            output.push(Box::new(
                TouchCommand {
                    path: String::from(line),
                    contents: String::from(""),
                }
            ));
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
        assert_eq!(&commands[0].to_string(), "mkdir -p /a/path/with/multiple/directories/");
    }

    #[test]
    fn test_is_dir_returns_false_for_invalid_directory() {
        assert!(!is_dir("example/file"));
    }

    #[test]
    fn test_is_dir_returns_true_for_valid_directory() {
        assert!(is_dir("example/folder/"));
    }
}
