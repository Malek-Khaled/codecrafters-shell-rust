use permissions::is_executable;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::Path;
use std::{env::var, path::PathBuf};

const BULITINS: [&str; 3] = ["echo", "exit", "type"];

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let args: Vec<&str> = command.trim().split(" ").collect();
        if args[0].eq("exit") {
            break;
        } else if args[0].eq("type") {
            type_com(&args);
            continue;
        } else if args[0].eq("echo") {
            echo(&args);
        } else if args[0].is_empty() {
            continue;
        } else if is_command_exist(args[0]) {
            let output = std::process::Command::new(args[0])
                .args(args[1..].iter())
                .output()
                .unwrap();

            io::stdout().write_all(&output.stdout).unwrap();
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

// shell bulitins
// -------------------------
fn type_com(commands: &[&str]) {
    let mut is_found = false;
    let paths = var("PATH").unwrap();
    for command in commands[1..].iter() {
        let paths_vec: Vec<&str> = paths.split(":").collect();
        for builtin in BULITINS {
            if command.eq(&builtin) {
                println!("{command} is a shell builtin");
                is_found = true;
                break;
            }
        }
        if !is_found {
            for path in paths_vec {
                let path = PathBuf::from(path).read_dir();
                if let Ok(path) = path {
                    for entry in path.flatten() {
                        let entry_string = entry.file_name().into_string().unwrap();
                        if entry_string.eq(command) && is_executable(entry.path()).unwrap() {
                            println!("{command} is {}", entry.path().to_str().unwrap());
                            is_found = true;
                            break;
                        }
                    }
                };
                if is_found {
                    break;
                }
            }
            if !is_found {
                println!("{command}: not found");
            }
        }
    }
}

fn echo(args: &[&str]) {
    for arg in args[1..].iter() {
        print!("{arg} ");
    }
    println!();
}
