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
}
