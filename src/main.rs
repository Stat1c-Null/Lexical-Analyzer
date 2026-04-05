use std::fs::File;
use std::io::{self, Read};

mod token;
mod lexer;

use lexer::lexer::Lexer;

fn main() {
    //Python lexical analyzer

    println!("Welcome to the Python Lexical Analyzer!");
    println!("Please select an option:\n1. Input Python code\n2. Use default code\n3. Analyze a file(TODO, not implemented yet)");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input = input.trim();// Trim \n

    if input == "1" { //TODO: Implement the logic for analyzing the input code
        println!("Enter your python code (multi-line supported). Press Ctrl+D(Linux) or Ctrl+Z(Windows) when done:");
        let mut code = String::new();
        io::stdin()
            .read_to_string(&mut code)
            .expect("Failed to read input");

        let mut lexer = Lexer::new(&code);
        let tokens = lexer.tokenize();
        for token in tokens {
            println!("{:?}", token);
        }

    } else if input == "2" {
        let default_code = "def add_numbers(num1, num2):
            sum = num1 + num2
            print('Sum: ',sum)";
        println!("Default code:\n{}\n", default_code);
        let mut lexer = Lexer::new(default_code);

        let tokens = lexer.tokenize();
        for token in tokens {
            println!("{:?}", token);
        }

    } else { //TODO: Implement the logic for analyzing the file
        println!("Enter path to the file:");
        let mut path = String::new();
        io::stdin()
            .read_line(&mut path)
            .expect("Failed to read input");

        let _file_input = File::open(path.trim()).unwrap();
    }
    
}