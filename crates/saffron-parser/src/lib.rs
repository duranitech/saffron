//! # Saffron Parser
//!
//! Recursive descent parser that produces an AST from a token stream.
//! The parser is hand-written for maximum error recovery and descriptive diagnostics.

use saffron_ast::*;
use saffron_lexer::{Token, TokenKind};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Expected {expected} but found {found} at line {line}")]
    UnexpectedToken {
        expected: String,
        found: String,
        line: u32,
    },

    #[error("Expected recipe block but found end of file")]
    UnexpectedEof,

    #[error("Invalid step number {number}: steps must be sequential starting from 1")]
    InvalidStepNumber { number: u32, line: u32 },
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    errors: Vec<ParseError>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
            errors: Vec::new(),
        }
    }

    /// Parse a complete recipe from the token stream
    pub fn parse_recipe(mut self) -> Result<(Recipe, Vec<ParseError>), Vec<ParseError>> {
        // TODO: Implement full parser in Phase 1
        // This is the scaffold showing the structure

        Err(self.errors)
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&self.tokens[self.tokens.len() - 1])
    }

    fn advance(&mut self) -> &Token {
        let token = &self.tokens[self.pos];
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        token
    }

    fn expect(&mut self, expected: TokenKind) -> Result<&Token, ParseError> {
        let token = self.peek();
        if std::mem::discriminant(&token.kind) == std::mem::discriminant(&expected) {
            Ok(self.advance())
        } else {
            Err(ParseError::UnexpectedToken {
                expected: format!("{:?}", expected),
                found: format!("{:?}", token.kind),
                line: token.span.start_line,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_creation() {
        let tokens = vec![Token {
            kind: TokenKind::Eof,
            span: Span {
                file: "test.saffron".into(),
                start_line: 1, start_col: 1,
                end_line: 1, end_col: 1,
                byte_offset: 0, byte_length: 0,
            },
            lexeme: String::new(),
        }];
        let parser = Parser::new(tokens);
        assert_eq!(parser.pos, 0);
    }
}
