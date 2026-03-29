use std::io;
use std::fs::File;

mod token;
mod lexer;

fn main() {
    //Python lexical analyzer
    let mut input = String :: new();
    println!("1) Manual input. \n2) Read from file");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    if input == "1" {
        println!("Enter your python code:");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        //let mut lexer = lexer::Lexer::new(&input);
    } else {
        println!("Enter path to the file:");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let fileInput = File::open(&input.trim()).unwrap();
    }
    
}

fn analyze_file(file: &str) -> i32 {
    // Implementation for analyzing the file
    return 0;
}