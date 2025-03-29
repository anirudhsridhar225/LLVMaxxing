use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    //keywords
    #[token("int")]
    Int,
    #[token("float")]
    Float,
    #[token("return")]
    Return,

    //identifiers
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    //literals
    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i32>().unwrap())]
    IntLiteral(i32),
    #[regex(r"[0-9]+\\.[0-9]+", |lex| lex.slice().parse::<f32>().unwrap())]
    FloatLiteral(f32),

    //operators
    #[token("+")]
    Plus,
    #[token("*")]
    Star,
    #[token("=")]
    Equals,
    #[token("/")]
    Divides,

    //punctuation
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token(",")]
    Comman,
    #[token("(")]
    OParen,
    #[token(")")]
    CParen,
    #[token("{")]
    OBrace,
    #[token("}")]
    CBrace,

    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

pub fn tokenize(source: &str) -> Vec<Token> {
    Token::lexer(source).filter_map(Result::ok).collect()
}
