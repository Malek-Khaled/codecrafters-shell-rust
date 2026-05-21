use permissions::is_executable;
use std::env::home_dir;
use std::env::set_current_dir;
use std::fmt;
use std::fmt::Display;
use std::{env::var, path::PathBuf};

const BULITINS: [&str; 5] = ["echo", "exit", "type", "pwd", "cd"];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ShellString {
    string: String,
}

impl Display for ShellString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.formatting())
    }
}
impl ShellString {
    fn formatting(&self) -> String {
        if !self.string.starts_with("\'") && !self.string.ends_with("\'") {
            self.string
                .replace("\"", "")
                .replace("\'", "")
                .replace("~", home_dir().unwrap().to_str().unwrap())
        } else {
            self.string.replace("\'", "").replace("\"", "")
        }
    }
    fn from(str: &str) -> ShellString {
        ShellString {
            string: String::from(str),
        }
    }
}

pub fn type_com(commands: &[&str]) {
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

pub fn echo(args: &[&str]) {
    for arg in args[1..].iter() {
        let shell_string = ShellString::from(arg);
        print!("{shell_string} ");
    }
    println!();
}

pub fn cd(args: &[&str]) {
    let path = if args.len() != 1 {
        let path_string = ShellString::from(args[1]);
        PathBuf::from(path_string.formatting())
    } else {
        home_dir().unwrap()
    };
    let result = set_current_dir(&path);
    if result.is_err() {
        println!("cd: {}: No such file or directory", path.display());
    }
}
