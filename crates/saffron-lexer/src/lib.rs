//! # Saffron Lexer
//!
//! Tokenizer for the Saffron programming language.
//! Converts source text into a stream of typed tokens with span information.
//!
//! Design decisions:
//! - Hand-written (not generated) for maximum error recovery and performance
//! - Zero-copy: tokens reference spans in the original source
//! - Unit literals (180.celsius) are lexed as single tokens
//! - Identifier casing is enforced at lex time

use saffron_ast::{Span, Unit};
use thiserror::Error;

/// Token types produced by the lexer
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literals
    IntLiteral(i64),
    FloatLiteral(f64),
    UnitLiteral { value: f64, unit: Unit },
    PercentLiteral(f64),
    StringLiteral(String),
    BoolLiteral(bool),

    // Identifiers (casing enforced)
    PascalIdent(String),   // Type names: Egg, FryingPan
    SnakeIdent(String),    // Variables: my_egg, oil_temp
    ScreamingIdent(String), // Constants: MAX_TEMP

    // Keywords
    Recipe,
    Ingredients,
    Equipment,
    Steps,
    ExpectedResult,
    Nutrition,
    Parallel,
    Let,
    Const,
    Mut,
    Fn,
    Async,
    Await,
    Return,
    If,
    Else,
    Match,
    For,
    While,
    In,
    Import,
    From,
    Export,
    Class,
    Abstract,
    Extends,
    Implements,
    Interface,
    Trait,
    Override,
    Readonly,
    New,
    True,
    False,
    Auto,

    // Punctuation
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Semicolon,
    Comma,
    Dot,
    Arrow,        // ->
    FatArrow,     // =>
    At,           // @

    // Operators
    Equal,        // ==
    NotEqual,     // !=
    LessThan,     // <
    LessEqual,    // <=
    GreaterThan,  // >
    GreaterEqual, // >=
    Assign,       // =
    Plus,
    Minus,
    Star,
    Slash,
    Percent,      // %

    // Special
    Newline,
    Comment(String),
    DocComment(String),
    AiHint(String),

    // Error recovery
    ErrorToken(String),

    // End of file
    Eof,
}

/// A token with its kind and source span
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub lexeme: String,
}

/// Lexer errors
#[derive(Debug, Error)]
pub enum LexError {
    #[error("Unexpected character '{ch}' at line {line}, column {col}")]
    UnexpectedChar { ch: char, line: u32, col: u32 },

    #[error("Unterminated string literal starting at line {line}")]
    UnterminatedString { line: u32 },

    #[error("Invalid unit suffix '{suffix}' at line {line}")]
    InvalidUnit { suffix: String, line: u32 },

    #[error("Invalid identifier casing: '{ident}' at line {line}. Expected {expected}")]
    InvalidCasing { ident: String, line: u32, expected: String },
}

/// The Saffron lexer
pub struct Lexer<'src> {
    source: &'src str,
    file: String,
    pos: usize,
    line: u32,
    col: u32,
    tokens: Vec<Token>,
    errors: Vec<LexError>,
}

impl<'src> Lexer<'src> {
    /// Create a new lexer for the given source text
    pub fn new(source: &'src str, file: impl Into<String>) -> Self {
        Self {
            source,
            file: file.into(),
            pos: 0,
            line: 1,
            col: 1,
            tokens: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// Tokenize the entire source, returning tokens and any errors
    pub fn tokenize(mut self) -> (Vec<Token>, Vec<LexError>) {
        while !self.is_at_end() {
            self.skip_whitespace();
            if self.is_at_end() {
                break;
            }
            self.scan_token();
        }

        self.tokens.push(Token {
            kind: TokenKind::Eof,
            span: self.current_span(0),
            lexeme: String::new(),
        });

        (self.tokens, self.errors)
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.source.len()
    }

    fn peek(&self) -> Option<char> {
        self.source[self.pos..].chars().next()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pos += ch.len_utf8();
        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(ch)
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch == ' ' || ch == '\t' || ch == '\r' || ch == '\n' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn current_span(&self, len: usize) -> Span {
        Span {
            file: self.file.clone(),
            start_line: self.line,
            start_col: self.col,
            end_line: self.line,
            end_col: self.col + len as u32,
            byte_offset: self.pos,
            byte_length: len,
        }
    }

    fn scan_token(&mut self) {
        // TODO: Implement full lexer
        // This is the scaffold — each match arm will be implemented in Phase 1
        let start_pos = self.pos;
        let start_line = self.line;
        let start_col = self.col;

        if let Some(ch) = self.advance() {
            let kind = match ch {
                '(' => TokenKind::LeftParen,
                ')' => TokenKind::RightParen,
                '{' => TokenKind::LeftBrace,
                '}' => TokenKind::RightBrace,
                '[' => TokenKind::LeftBracket,
                ']' => TokenKind::RightBracket,
                ':' => TokenKind::Colon,
                ';' => TokenKind::Semicolon,
                ',' => TokenKind::Comma,
                '@' => TokenKind::At,
                '+' => TokenKind::Plus,
                '*' => TokenKind::Star,
                '/' => {
                    if self.peek() == Some('/') {
                        self.advance();
                        // Line comment
                        let mut comment = String::new();
                        while let Some(c) = self.peek() {
                            if c == '\n' { break; }
                            comment.push(c);
                            self.advance();
                        }
                        TokenKind::Comment(comment)
                    } else {
                        TokenKind::Slash
                    }
                }
                _ => {
                    self.errors.push(LexError::UnexpectedChar {
                        ch,
                        line: start_line,
                        col: start_col,
                    });
                    TokenKind::ErrorToken(ch.to_string())
                }
            };

            let lexeme = self.source[start_pos..self.pos].to_string();
            let span = Span {
                file: self.file.clone(),
                start_line,
                start_col,
                end_line: self.line,
                end_col: self.col,
                byte_offset: start_pos,
                byte_length: self.pos - start_pos,
            };

            self.tokens.push(Token { kind, span, lexeme });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_source() {
        let lexer = Lexer::new("", "test.saffron");
        let (tokens, errors) = lexer.tokenize();
        assert_eq!(tokens.len(), 1); // just EOF
        assert_eq!(tokens[0].kind, TokenKind::Eof);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_punctuation() {
        let lexer = Lexer::new("(){}", "test.saffron");
        let (tokens, errors) = lexer.tokenize();
        assert!(errors.is_empty());
        assert_eq!(tokens[0].kind, TokenKind::LeftParen);
        assert_eq!(tokens[1].kind, TokenKind::RightParen);
        assert_eq!(tokens[2].kind, TokenKind::LeftBrace);
        assert_eq!(tokens[3].kind, TokenKind::RightBrace);
    }

    #[test]
    fn test_comment() {
        let lexer = Lexer::new("// this is a comment", "test.saffron");
        let (tokens, errors) = lexer.tokenize();
        assert!(errors.is_empty());
        assert!(matches!(tokens[0].kind, TokenKind::Comment(_)));
    }
}
