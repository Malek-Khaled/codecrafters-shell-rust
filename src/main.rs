use permissions::is_executable;
use shell_builtins::*;
use std::env::current_dir;
use std::env::var;
use std::io::{self, Write};
use std::path::Path;
mod shell_builtins;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let args: Vec<&str> = command
            .trim()
            .split(" ")
            .filter(|x| !x.is_empty())
            .collect();

        match args[0] {
            "exit" => break,
            "type" => type_com(&args),
            "echo" => echo(&args),
            "pwd" => println!("{}", current_dir().unwrap().to_str().unwrap()),
            "cd" => cd(&args),
            "" => continue,
            _ => {
                if is_command_exist(args[0]) {
                    let output = std::process::Command::new(args[0])
                        .args(args[1..].iter())
                        .output()
                        .unwrap();

                    io::stdout().write_all(&output.stdout).unwrap();
                }
            }
        }
    }
}

fn is_command_exist(command: &str) -> bool {
    let paths = var("PATH").unwrap();
    let paths_vec: Vec<&str> = paths.split(":").collect();
    for path in paths_vec {
        let bins_path = Path::new(path).read_dir();
        if let Ok(bins_path) = bins_path {
            for entry in bins_path.flatten() {
                let entry_string = entry.file_name().into_string().unwrap();
                if entry_string.eq(command) && is_executable(entry.path()).unwrap() {
                    return true;
                }
            }
        }
    }
    println!("{}: command not found", command);
    false
}
