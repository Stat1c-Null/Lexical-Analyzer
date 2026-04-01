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
pub enum Token {
    Identifier(String),
    Keyword(String),
    Literal(String),
    Operator(char),
    Delimiter(char),
    Whitespace,
}