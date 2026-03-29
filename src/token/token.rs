#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    Identifier(String),
    Keyword(String),
    Literal(String),
    Operator(char),
    Delimiter(char),
    Whitespace,
}