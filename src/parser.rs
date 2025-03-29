use lalrpop_util::lalrpop_mod;
use lalrpop_util::ParseError;

lalrpop_mod!(pub grammar);

use crate::ast;
use crate::lexer::Token;

pub fn parse(tokens: &[Token]) -> Result<ast::Program, String> {
    let parser = grammar::ProgramParser::new();
    let token_triples = tokens
        .iter()
        .enumerate()
        .map(|(i, token)| (i, token.clone(), i + 1));

    parser.parse(token_triples).map_err(|e| match e {
        ParseError::InvalidToken { location } => {
            format!("Invalid token at position {}", location)
        }
        ParseError::UnrecognizedEof { location, expected } => {
            format!(
                "Unexpected end of file at position {}. Expected: {:?}",
                location, expected
            )
        }
        ParseError::UnrecognizedToken {
            token: (start, token, _end),
            expected,
        } => {
            format!(
                "Unexpected token '{:?}' at position {}. Expected one of: {:?}",
                token, start, expected
            )
        }
        ParseError::ExtraToken {
            token: (start, token, _),
        } => {
            format!("Extra token '{:?}' at position {}", token, start)
        }
        ParseError::User { error } => {
            format!("Custom error: {}", error)
        }
    })
}
