mod proj_struct;

fn main() {
    let commands = proj_struct::parse("/a/path/with/multiple/directories/".to_string());
    for cmd in commands {
        println!("would have executed: {}", cmd.args[0]);
    }
}
