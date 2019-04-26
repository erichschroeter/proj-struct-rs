#[derive(Debug)]
pub struct Command {
    pub args: Vec<String>,
}

impl Command {
    pub fn new() -> Self {
        Command { args: Vec::new() }
    }

    pub fn add_arg(&mut self, arg: &str) {
        self.args.push(arg.to_string());
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

pub fn parse(input_string: String) -> Vec<Command> {
    let mut output: Vec<Command> = vec![];
    let lines = input_string.split("\n");

    for line in lines {
        let mut cmd = Command::new();
        cmd.add_arg(line);
        output.push(cmd);
        println!("{}", line);
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let commands = parse("/a/path/with/multiple/directories/".to_string());
        assert_eq!(&commands[0].args[0], "/a/path/with/multiple/directories/");
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
