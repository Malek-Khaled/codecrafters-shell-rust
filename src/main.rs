#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::Path;

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();
    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();
    is_command_exist(&command);
}

fn is_command_exist(command: &str) -> bool {
    let bin_path = Path::new("/bin").read_dir().unwrap();
    for entry in bin_path.flatten() {
        let entry_string = entry.file_name().into_string().unwrap();
        if entry_string.eq(command.trim()) {
            return true;
        }
    }
    println!("{}: command not found", command.trim());
    false
}
