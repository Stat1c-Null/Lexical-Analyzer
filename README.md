# Lexical-Analyzer
Python Lexical Analyzer written in Rust 

## References
[SLang-Rust-Lexical-Analyzer](https://github.com/GraHms/SLang-Rust-Lexical-Analyzer/tree/master) helped me design this lexical analyzer 

## Functionality
- Analyze Python code entered by the user manually
- Analyze default Python code stored in a variable
- Analyze Python code from a file (In progress)

## Installation
```bash```
cargo build

## Usage
cargo run

## Input
print("test")
test = 10
def test():
    return 5 + 10

## How It Works
1. Read input
2. Tokenize into characters
3. Match tokens (IDENT, INT, etc.)
4. Output token stream
