use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    println!("welcome to shex");
    loop {
        // display the prompt
        let _ = get_current_dir();
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
        match args[0] {
            "exit" => break,
            "cd" => {
                let _ = env::set_current_dir(args[1]);
            }
            _ => {
                if args[0].ends_with(".sh") {
                    run_script(args[0]);
                    continue;
                } else {
                    execute_command(&args);
                }
            }
        }
    }
    println!("exited");
}

fn execute_command(args: &[&str]) {
    let mut command = Command::new(args[0]);
    command.args(&args[1..]);
    match command.status() {
        Ok(status) => {
            if !status.success() {
                println!("command exited with {}", status)
            }
        }
        Err(_) => {
            println!("command '{}' not found", args[0]);
        }
    }
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

fn get_current_dir() -> std::io::Result<()> {
    let path = env::current_dir()?;
    let home = env::var("HOME").unwrap();

    let path = path.display().to_string();

    if path.starts_with(&home) {
        println!("~{}", &path[home.len()..])
    } else {
        println!("{}", path)
    }
    Ok(())
}

// TODO: add "echo > file.txt" and split the code to multiple function
