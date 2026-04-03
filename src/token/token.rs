#[derive(Debug, Clone, PartialEq, Eq, Hash)]

pub enum TokenType {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT,
    INT,
    FLOAT,
    STRING,

    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    PERCENT,
    EQ,
    NOTEQ,
    LT,
    GT,
    LTE,
    GTE,

    // Delimiters
    COMMA,
    COLON,
    DOT,
    LPAREN,
    RPAREN,
    LBRACKET,
    RBRACKET,

    // Python-specific
    NEWLINE,
    INDENT,
    DEDENT,

    // Keywords
    DEF,
    RETURN,
    IF,
    ELSE,
    ELIF,
    FOR,
    WHILE,
    BREAK,
    CONTINUE,
    PASS,
    TRUE,
    FALSE,
    NONE,
    AND,
    OR,
    NOT,
    IN,
    IS,
    CLASS,
    IMPORT,
    FROM,
    AS,
    WITH,
    TRY,
    EXCEPT,
    FINALLY,
    RAISE,
    YIELD,
    LAMBDA,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    pub r#type: TokenType,
    pub literal: String,
    pub line: i32, 
    pub column: i32
}

pub fn lookup_ident(ident: &str) -> TokenType {
    match ident {
        "def" => TokenType::DEF,
        "return" => TokenType::RETURN,
        "if" => TokenType::IF,
        "else" => TokenType::ELSE,
        "elif" => TokenType::ELIF,
        "for" => TokenType::FOR,
        "while" => TokenType::WHILE,
        "break" => TokenType::BREAK,
        "continue" => TokenType::CONTINUE,
        "pass" => TokenType::PASS,
        "True" => TokenType::TRUE,
        "False" => TokenType::FALSE,
        "None" => TokenType::NONE,
        "and" => TokenType::AND,
        "or" => TokenType::OR,
        "not" => TokenType::NOT,
        "in" => TokenType::IN,
        "is" => TokenType::IS,
        "class" => TokenType::CLASS,
        "import" => TokenType::IMPORT,
        "from" => TokenType::FROM,
        "as" => TokenType::AS,
        "with" => TokenType::WITH,
        "try" => TokenType::TRY,
        "except" => TokenType::EXCEPT,
        "finally" => TokenType::FINALLY,
        "raise" => TokenType::RAISE,
        "yield" => TokenType::YIELD,
        "lambda" => TokenType::LAMBDA,
        _ => TokenType::IDENT
    }
}