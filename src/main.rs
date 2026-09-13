use std::io::{self, Write};
use std::process::{Command};


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
        
        // execute commands
        let mut command = Command::new(args[0]);
        command.args(&args[1..]);
        let _ = command.status();
    }
    println!("exit");
}

// TODO: add "echo > file.txt" and split the code to multiple function
