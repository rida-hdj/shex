use std::fs;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    println!("welcome to shex");
    loop {
        // display the prompt "> "
        print!("> ");
        io::stdout().flush().unwrap();

        // take input
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        // clean the input
        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        // split the input into arguments
        let args: Vec<&str> = input.split_whitespace().collect();

        // exit
        if args[0] == "exit" {
            break;
        }

        if args[0].ends_with(".sh") {
            run_script(args[0]);
            continue;
        }

        // execute commands
        execute_command(&args);
    }
    println!("exit");
}

fn execute_command(args: &[&str]) {
    let mut command = Command::new(args[0]);
    command.args(&args[1..]);
    let _ = command.status();
}

fn run_script(path: &str) {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(log) => {
            println!("can't read file {} in {}", log, path);
            return;
        }
    };
    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with("#") {
            continue;
        }

        let args: Vec<&str> = line.split_whitespace().collect();
        execute_command(&args);
    }
}

// TODO: add "echo > file.txt" and split the code to multiple function
