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
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse::<f32>().unwrap())]
    FloatLiteral(f32),

    //include files
    #[regex(r"#include\s*<([^>]+)>", |lex| lex.slice()[10..lex.slice().len()-1].to_string())]
    #[regex(r#"#include\s*"([^"]+)"#, |lex| lex.slice()[10..lex.slice().len()-1].to_string())]
    Include(String),

    //operators
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("=")]
    Equals,
    #[token("/")]
    FSlash,

    //punctuation
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token("(")]
    OParen,
    #[token(")")]
    CParen,
    #[token("{")]
    OBrace,
    #[token("}")]
    CBrace,

    //string literal
    #[regex(r#""([^"\\]|\\["\\nt])*""#, |lex| {
        let s = lex.slice();
        s[1..s.len()-1].to_string()
    })]
    StringLiteral(String),


    //printing and scanning
    #[token("printf")]
    Printf,
    #[token("scanf")]
    Scanf,

    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

pub fn tokenize(source: &str) -> Vec<Token> {
    Token::lexer(source).filter_map(Result::ok).collect()
}
