use std::io;
use std::fs::File;

mod token;
mod lexer;

use lexer::lexer::Lexer;

fn main() {
    //Python lexical analyzer

    println!("Welcome to the Python Lexical Analyzer!");
    println!("Please select an option:\n1. Input Python code\n2. Use default code\n3. Analyze a file");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input = input.trim();// Trim \n

    if input == "1" { //TODO: Implement the logic for analyzing the input code
        println!("Enter your python code:");
        let mut code = String::new();
        io::stdin()
            .read_line(&mut code)
            .expect("Failed to read input");

    } else if input == "2" {
        let default_code = "def add_numbers(num1, num2):
                                        sum = num1 + num2
                                        print('Sum: ',sum)";
        println!("Default code:\n{}", default_code);
        let lexer = Lexer::new(default_code);

    } else { //TODO: Implement the logic for analyzing the file
        println!("Enter path to the file:");
        let mut path = String::new();
        io::stdin()
            .read_line(&mut path)
            .expect("Failed to read input");

        let file_input = File::open(path.trim()).unwrap();
    }
    
}

fn analyze_file(file: &str) -> i32 {
    // Implementation for analyzing the file
    return 0;
}