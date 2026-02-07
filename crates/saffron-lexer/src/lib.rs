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
//! - Error recovery: invalid characters produce ErrorToken, lexing continues

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
    PascalIdent(String),    // Type names: Egg, FryingPan
    SnakeIdent(String),     // Variables: my_egg, oil_temp
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
    Arrow,    // ->
    FatArrow, // =>
    At,       // @

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
    Percent, // %

    // Special — Newline is reserved for future significant-newline support.
    // Currently unused: the lexer silently skips whitespace including '\n'.
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
    InvalidCasing {
        ident: String,
        line: u32,
        expected: String,
    },

    #[error("Invalid unicode escape sequence at line {line}")]
    InvalidUnicodeEscape { line: u32 },
}

// ---------------------------------------------------------------------------
// Unit suffix lookup table
// Ordered longest-first within each prefix group to prevent partial matches
// in sequential scanning (same ordering principle as PEG grammar §16).
// ---------------------------------------------------------------------------

/// Try to match a unit suffix at the beginning of `s`.
/// Returns the matched Unit and the byte length consumed, or None.
fn match_unit_suffix(s: &str) -> Option<(Unit, usize)> {
    macro_rules! try_suffix {
        ($s:expr, $suffix:expr, $unit:expr) => {
            if $s.starts_with($suffix) {
                let len = $suffix.len();
                let at_boundary = $s[len..]
                    .chars()
                    .next()
                    .map_or(true, |c| !c.is_alphanumeric() && c != '_');
                if at_boundary {
                    return Some(($unit, len));
                }
            }
        };
    }

    // Temperature
    try_suffix!(s, "fahrenheit", Unit::Fahrenheit);
    try_suffix!(s, "celsius", Unit::Celsius);
    try_suffix!(s, "kelvin", Unit::Kelvin);
    // Mass (longest first)
    try_suffix!(s, "milligrams", Unit::Milligrams);
    try_suffix!(s, "kilograms", Unit::Kilograms);
    try_suffix!(s, "grams", Unit::Grams);
    try_suffix!(s, "ounces", Unit::Ounces);
    try_suffix!(s, "pounds", Unit::Pounds);
    // Volume (longest first)
    try_suffix!(s, "milliliters", Unit::Milliliters);
    try_suffix!(s, "fluid_ounces", Unit::FluidOunces);
    try_suffix!(s, "tablespoons", Unit::Tablespoons);
    try_suffix!(s, "teaspoons", Unit::Teaspoons);
    try_suffix!(s, "liters", Unit::Liters);
    try_suffix!(s, "cups", Unit::Cups);
    // Time (longest first)
    try_suffix!(s, "minutes", Unit::Minutes);
    try_suffix!(s, "seconds", Unit::Seconds);
    try_suffix!(s, "hours", Unit::Hours);
    // Length (longest first)
    try_suffix!(s, "centimeters", Unit::Centimeters);
    try_suffix!(s, "millimeters", Unit::Millimeters);
    try_suffix!(s, "inches", Unit::Inches);
    // Energy (longest first)
    try_suffix!(s, "kilocalories", Unit::Kilocalories);
    try_suffix!(s, "calories", Unit::Calories);
    try_suffix!(s, "joules", Unit::Joules);
    // Power
    try_suffix!(s, "watts", Unit::Watts);
    // Percentage
    try_suffix!(s, "percent", Unit::Percent);
    // Abbreviations — MUST come after full forms sharing a prefix
    try_suffix!(s, "ml", Unit::Milliliters);
    try_suffix!(s, "cm", Unit::Centimeters);
    try_suffix!(s, "mm", Unit::Millimeters);

    None
}

// ---------------------------------------------------------------------------
// Keyword lookup
// ---------------------------------------------------------------------------

fn lookup_keyword(ident: &str) -> Option<TokenKind> {
    match ident {
        // Phase 1
        "recipe" => Some(TokenKind::Recipe),
        "ingredients" => Some(TokenKind::Ingredients),
        "equipment" => Some(TokenKind::Equipment),
        "steps" => Some(TokenKind::Steps),
        "expected_result" => Some(TokenKind::ExpectedResult),
        "nutrition" => Some(TokenKind::Nutrition),
        "parallel" => Some(TokenKind::Parallel),
        "import" => Some(TokenKind::Import),
        "from" => Some(TokenKind::From),
        "auto" => Some(TokenKind::Auto),
        "true" => Some(TokenKind::BoolLiteral(true)),
        "false" => Some(TokenKind::BoolLiteral(false)),
        // Phase 2+
        "fn" => Some(TokenKind::Fn),
        "let" => Some(TokenKind::Let),
        "const" => Some(TokenKind::Const),
        "mut" => Some(TokenKind::Mut),
        "return" => Some(TokenKind::Return),
        "if" => Some(TokenKind::If),
        "else" => Some(TokenKind::Else),
        "match" => Some(TokenKind::Match),
        "for" => Some(TokenKind::For),
        "while" => Some(TokenKind::While),
        "in" => Some(TokenKind::In),
        "async" => Some(TokenKind::Async),
        "await" => Some(TokenKind::Await),
        "export" => Some(TokenKind::Export),
        "class" => Some(TokenKind::Class),
        "abstract" => Some(TokenKind::Abstract),
        "extends" => Some(TokenKind::Extends),
        "implements" => Some(TokenKind::Implements),
        "interface" => Some(TokenKind::Interface),
        "trait" => Some(TokenKind::Trait),
        "override" => Some(TokenKind::Override),
        "readonly" => Some(TokenKind::Readonly),
        "new" => Some(TokenKind::New),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Identifier casing classification
// ---------------------------------------------------------------------------

/// Classify an identifier by its casing pattern.
///
/// Rules (from grammar.peg §12):
///   SCREAM <- [A-Z][A-Z0-9_][A-Z0-9_]+   (3+ chars, all uppercase/digits/underscores)
///   PASCAL <- [A-Z][a-zA-Z0-9]*           (starts uppercase, mixed case allowed)
///   SNAKE  <- [a-z][a-z0-9_]*             (starts lowercase, underscores allowed)
fn classify_identifier(ident: &str) -> TokenKind {
    let first = ident.chars().next().unwrap();

    if first.is_ascii_uppercase() {
        // SCREAM requires: 3+ chars, all [A-Z0-9_]
        let is_screaming = ident.len() >= 3
            && ident
                .chars()
                .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_');
        if is_screaming {
            TokenKind::ScreamingIdent(ident.to_string())
        } else {
            TokenKind::PascalIdent(ident.to_string())
        }
    } else {
        // Starts with lowercase → snake_case
        TokenKind::SnakeIdent(ident.to_string())
    }
}

// ===========================================================================
// Lexer
// ===========================================================================

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
            span: self.make_span(self.pos, self.pos),
            lexeme: String::new(),
        });

        (self.tokens, self.errors)
    }

    // -----------------------------------------------------------------------
    // Core helpers
    // -----------------------------------------------------------------------

    fn is_at_end(&self) -> bool {
        self.pos >= self.source.len()
    }

    fn peek(&self) -> Option<char> {
        self.source[self.pos..].chars().next()
    }

    /// Look ahead by one character past the current peek position.
    /// Reserved for parser-assisted re-lexing and future multi-char lookahead.
    #[allow(dead_code)]
    fn peek_next(&self) -> Option<char> {
        let mut chars = self.source[self.pos..].chars();
        chars.next(); // skip current
        chars.next()
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

    /// Conditionally consume the next character if it matches `expected`.
    fn match_char(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
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

    fn make_span(&self, start: usize, end: usize) -> Span {
        // Compute start line/col by scanning from beginning (expensive but correct).
        // For production, we'd track these incrementally. But start_line/start_col
        // are captured at scan_token entry, so this is only used for EOF.
        Span {
            file: self.file.clone(),
            start_line: self.line,
            start_col: self.col,
            end_line: self.line,
            end_col: self.col,
            byte_offset: start,
            byte_length: end - start,
        }
    }

    fn emit(&mut self, kind: TokenKind, start_pos: usize, start_line: u32, start_col: u32) {
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

    // -----------------------------------------------------------------------
    // Main dispatcher
    // -----------------------------------------------------------------------

    fn scan_token(&mut self) {
        let start_pos = self.pos;
        let start_line = self.line;
        let start_col = self.col;

        if let Some(ch) = self.advance() {
            let kind = match ch {
                // Grouping
                '(' => TokenKind::LeftParen,
                ')' => TokenKind::RightParen,
                '{' => TokenKind::LeftBrace,
                '}' => TokenKind::RightBrace,
                '[' => TokenKind::LeftBracket,
                ']' => TokenKind::RightBracket,

                // Punctuation
                ':' => TokenKind::Colon,
                ';' => TokenKind::Semicolon,
                ',' => TokenKind::Comma,
                '@' => TokenKind::At,
                '.' => TokenKind::Dot,

                // Simple operators
                '+' => TokenKind::Plus,
                '*' => TokenKind::Star,

                // Multi-char operators
                '-' => {
                    if self.match_char('>') {
                        TokenKind::Arrow
                    } else {
                        TokenKind::Minus
                    }
                }
                '=' => {
                    if self.match_char('=') {
                        TokenKind::Equal
                    } else if self.match_char('>') {
                        TokenKind::FatArrow
                    } else {
                        TokenKind::Assign
                    }
                }
                '!' => {
                    if self.match_char('=') {
                        TokenKind::NotEqual
                    } else {
                        self.errors.push(LexError::UnexpectedChar {
                            ch: '!',
                            line: start_line,
                            col: start_col,
                        });
                        TokenKind::ErrorToken("!".to_string())
                    }
                }
                '<' => {
                    if self.match_char('=') {
                        TokenKind::LessEqual
                    } else {
                        TokenKind::LessThan
                    }
                }
                '>' => {
                    if self.match_char('=') {
                        TokenKind::GreaterEqual
                    } else {
                        TokenKind::GreaterThan
                    }
                }
                '%' => TokenKind::Percent,

                // Slash or comment
                '/' => {
                    if self.peek() == Some('/') {
                        self.advance(); // consume second '/'
                        self.scan_comment()
                    } else {
                        TokenKind::Slash
                    }
                }

                // String literal
                '"' => self.scan_string(start_line),

                // Numeric literal (may become UnitLiteral or PercentLiteral)
                c if c.is_ascii_digit() => self.scan_number(start_pos),

                // Identifier or keyword
                c if c.is_ascii_alphabetic() || c == '_' => {
                    self.scan_identifier_or_keyword(start_pos)
                }

                // Unknown character → error recovery
                _ => {
                    self.errors.push(LexError::UnexpectedChar {
                        ch,
                        line: start_line,
                        col: start_col,
                    });
                    TokenKind::ErrorToken(ch.to_string())
                }
            };

            self.emit(kind, start_pos, start_line, start_col);
        }
    }

    // -----------------------------------------------------------------------
    // Number scanning
    //
    // Handles: IntLiteral, FloatLiteral, UnitLiteral, PercentLiteral
    //
    // Algorithm:
    //   1. Read integer digits
    //   2. If '.' followed by digit → float (consume '.' + digits)
    //   3. After number, if '.' followed by unit suffix → UnitLiteral
    //   4. After number, if '%' → PercentLiteral
    //   5. Otherwise → IntLiteral or FloatLiteral
    //
    // Parse traces:
    //   "180.celsius" → UnitLiteral(180.0, Celsius)
    //   "2.5.cm"      → UnitLiteral(2.5, Centimeters)
    //   "3.14"        → FloatLiteral(3.14)
    //   "42"          → IntLiteral(42)
    //   "76%"         → PercentLiteral(76.0)
    // -----------------------------------------------------------------------

    fn scan_number(&mut self, start_pos: usize) -> TokenKind {
        // First digit already consumed by advance() in scan_token.
        // Read remaining integer digits.
        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.advance();
        }

        let mut is_float = false;

        // Check for decimal point → float
        // Only if the char after '.' is a digit (otherwise '.' is unit separator or DOT)
        if self.peek() == Some('.') {
            let after_dot = self
                .source
                .get(self.pos + 1..)
                .and_then(|s| s.chars().next());
            if after_dot.is_some_and(|c| c.is_ascii_digit()) {
                self.advance(); // consume '.'
                while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                    self.advance();
                }
                is_float = true;
            }
        }

        // Parse the numeric value
        let num_str = &self.source[start_pos..self.pos];
        let value: f64 = num_str.parse().unwrap_or(0.0);

        // Check for unit suffix: NUMBER '.' UNIT_SUFFIX !ID_CHAR
        if self.peek() == Some('.') {
            let after_dot = self.source.get(self.pos + 1..).unwrap_or("");
            if let Some((unit, suffix_len)) = match_unit_suffix(after_dot) {
                self.advance(); // consume '.'
                for _ in 0..suffix_len {
                    self.advance(); // consume suffix chars
                }
                return TokenKind::UnitLiteral { value, unit };
            }
        }

        // Check for percent: NUMBER '%'
        if self.peek() == Some('%') {
            self.advance(); // consume '%'
            return TokenKind::PercentLiteral(value);
        }

        // Plain number
        if is_float {
            TokenKind::FloatLiteral(value)
        } else {
            TokenKind::IntLiteral(value as i64)
        }
    }

    // -----------------------------------------------------------------------
    // String scanning
    //
    // Double-quoted with escape sequences: \" \\ \/ \b \f \n \r \t \uXXXX
    // Opening quote already consumed by scan_token.
    // -----------------------------------------------------------------------

    fn scan_string(&mut self, string_start_line: u32) -> TokenKind {
        let mut value = String::new();

        loop {
            match self.advance() {
                None => {
                    self.errors.push(LexError::UnterminatedString {
                        line: string_start_line,
                    });
                    return TokenKind::ErrorToken(format!("\"{value}"));
                }
                Some('"') => break,
                Some('\\') => match self.advance() {
                    Some('"') => value.push('"'),
                    Some('\\') => value.push('\\'),
                    Some('/') => value.push('/'),
                    Some('b') => value.push('\u{0008}'),
                    Some('f') => value.push('\u{000C}'),
                    Some('n') => value.push('\n'),
                    Some('r') => value.push('\r'),
                    Some('t') => value.push('\t'),
                    Some('u') => {
                        let mut hex = String::with_capacity(4);
                        for _ in 0..4 {
                            match self.advance() {
                                Some(c) if c.is_ascii_hexdigit() => hex.push(c),
                                _ => {
                                    self.errors.push(LexError::InvalidUnicodeEscape {
                                        line: self.line,
                                    });
                                    return TokenKind::ErrorToken(format!("\"{value}"));
                                }
                            }
                        }
                        if let Ok(code) = u32::from_str_radix(&hex, 16) {
                            if let Some(ch) = char::from_u32(code) {
                                value.push(ch);
                            }
                        }
                    }
                    Some(c) => {
                        // Unknown escape — include literally for error recovery
                        value.push('\\');
                        value.push(c);
                    }
                    None => {
                        self.errors.push(LexError::UnterminatedString {
                            line: string_start_line,
                        });
                        return TokenKind::ErrorToken(format!("\"{value}"));
                    }
                },
                Some(c) => value.push(c),
            }
        }

        TokenKind::StringLiteral(value)
    }

    // -----------------------------------------------------------------------
    // Comment scanning
    //
    // Called after '//' has been consumed. Distinguishes:
    //   //           → Comment       (grammar: '//' !'/' ...)
    //   ///          → DocComment    (grammar: '///' !'ai:' ...)
    //   ///ai:       → AiHint       (grammar: '///ai:' ...)
    // -----------------------------------------------------------------------

    fn scan_comment(&mut self) -> TokenKind {
        // Check for third '/'
        if self.peek() == Some('/') {
            self.advance(); // consume third '/'

            // Check for 'ai:' immediately after ///
            if self.source.get(self.pos..).is_some_and(|s| s.starts_with("ai:")) {
                // Consume 'ai:'
                self.advance(); // a
                self.advance(); // i
                self.advance(); // :

                let content = self.read_until_eol();
                return TokenKind::AiHint(content);
            }

            // Doc comment
            let content = self.read_until_eol();
            return TokenKind::DocComment(content);
        }

        // Regular comment (two slashes only)
        let content = self.read_until_eol();
        TokenKind::Comment(content)
    }

    /// Read all characters until end of line (exclusive). Does NOT consume '\n'.
    fn read_until_eol(&mut self) -> String {
        let start = self.pos;
        while let Some(c) = self.peek() {
            if c == '\n' {
                break;
            }
            self.advance();
        }
        let raw = &self.source[start..self.pos];
        // Content is returned verbatim (including any leading space after //)
        // so callers can decide how to handle whitespace.
        raw.to_string()
    }

    // -----------------------------------------------------------------------
    // Identifier and keyword scanning
    //
    // Reads [a-zA-Z_][a-zA-Z0-9_]* then:
    //   1. Checks keyword table
    //   2. Classifies by casing: SCREAM / PASCAL / SNAKE
    // -----------------------------------------------------------------------

    fn scan_identifier_or_keyword(&mut self, start_pos: usize) -> TokenKind {
        // First char already consumed. Read remaining identifier chars.
        while self.peek().is_some_and(|c| c.is_alphanumeric() || c == '_') {
            self.advance();
        }

        let ident = &self.source[start_pos..self.pos];

        // Keywords take priority
        if let Some(kw) = lookup_keyword(ident) {
            return kw;
        }

        // Classify by casing
        classify_identifier(ident)
    }
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(source: &str) -> (Vec<Token>, Vec<LexError>) {
        Lexer::new(source, "test.saffron").tokenize()
    }

    /// Extract just the token kinds (excluding Eof) for concise assertions.
    fn kinds(source: &str) -> Vec<TokenKind> {
        let (tokens, _) = lex(source);
        tokens
            .into_iter()
            .filter(|t| t.kind != TokenKind::Eof)
            .map(|t| t.kind)
            .collect()
    }

    // -----------------------------------------------------------------------
    // Basic tokens
    // -----------------------------------------------------------------------

    #[test]
    fn test_empty_source() {
        let (tokens, errors) = lex("");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::Eof);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_punctuation() {
        let k = kinds("(){}[]:;,@.");
        assert_eq!(
            k,
            vec![
                TokenKind::LeftParen,
                TokenKind::RightParen,
                TokenKind::LeftBrace,
                TokenKind::RightBrace,
                TokenKind::LeftBracket,
                TokenKind::RightBracket,
                TokenKind::Colon,
                TokenKind::Semicolon,
                TokenKind::Comma,
                TokenKind::At,
                TokenKind::Dot,
            ]
        );
    }

    #[test]
    fn test_operators() {
        let k = kinds("+ - * / % -> => == != <= >= < > = !");
        assert_eq!(
            k,
            vec![
                TokenKind::Plus,
                TokenKind::Minus,
                TokenKind::Star,
                TokenKind::Slash,
                TokenKind::Percent,
                TokenKind::Arrow,
                TokenKind::FatArrow,
                TokenKind::Equal,
                TokenKind::NotEqual,
                TokenKind::LessEqual,
                TokenKind::GreaterEqual,
                TokenKind::LessThan,
                TokenKind::GreaterThan,
                TokenKind::Assign,
                TokenKind::ErrorToken("!".to_string()),
            ]
        );
    }

    // -----------------------------------------------------------------------
    // Numbers
    // -----------------------------------------------------------------------

    #[test]
    fn test_integer() {
        assert_eq!(kinds("42"), vec![TokenKind::IntLiteral(42)]);
        assert_eq!(kinds("0"), vec![TokenKind::IntLiteral(0)]);
        assert_eq!(kinds("999"), vec![TokenKind::IntLiteral(999)]);
    }

    #[test]
    fn test_float() {
        assert_eq!(kinds("3.14"), vec![TokenKind::FloatLiteral(3.14)]);
        assert_eq!(kinds("0.5"), vec![TokenKind::FloatLiteral(0.5)]);
    }

    #[test]
    fn test_unit_literal_integer() {
        assert_eq!(
            kinds("180.celsius"),
            vec![TokenKind::UnitLiteral {
                value: 180.0,
                unit: Unit::Celsius
            }]
        );
        assert_eq!(
            kinds("50.ml"),
            vec![TokenKind::UnitLiteral {
                value: 50.0,
                unit: Unit::Milliliters
            }]
        );
        assert_eq!(
            kinds("2000.watts"),
            vec![TokenKind::UnitLiteral {
                value: 2000.0,
                unit: Unit::Watts
            }]
        );
    }

    #[test]
    fn test_unit_literal_float() {
        assert_eq!(
            kinds("2.5.cm"),
            vec![TokenKind::UnitLiteral {
                value: 2.5,
                unit: Unit::Centimeters
            }]
        );
        assert_eq!(
            kinds("300.0.grams"),
            vec![TokenKind::UnitLiteral {
                value: 300.0,
                unit: Unit::Grams
            }]
        );
    }

    #[test]
    fn test_unit_literal_all_suffixes() {
        let cases = vec![
            ("1.celsius", Unit::Celsius),
            ("1.fahrenheit", Unit::Fahrenheit),
            ("1.kelvin", Unit::Kelvin),
            ("1.grams", Unit::Grams),
            ("1.kilograms", Unit::Kilograms),
            ("1.ounces", Unit::Ounces),
            ("1.pounds", Unit::Pounds),
            ("1.milligrams", Unit::Milligrams),
            ("1.milliliters", Unit::Milliliters),
            ("1.liters", Unit::Liters),
            ("1.cups", Unit::Cups),
            ("1.tablespoons", Unit::Tablespoons),
            ("1.teaspoons", Unit::Teaspoons),
            ("1.fluid_ounces", Unit::FluidOunces),
            ("1.seconds", Unit::Seconds),
            ("1.minutes", Unit::Minutes),
            ("1.hours", Unit::Hours),
            ("1.centimeters", Unit::Centimeters),
            ("1.millimeters", Unit::Millimeters),
            ("1.inches", Unit::Inches),
            ("1.joules", Unit::Joules),
            ("1.calories", Unit::Calories),
            ("1.kilocalories", Unit::Kilocalories),
            ("1.watts", Unit::Watts),
            ("1.percent", Unit::Percent),
            // Abbreviations
            ("1.ml", Unit::Milliliters),
            ("1.cm", Unit::Centimeters),
            ("1.mm", Unit::Millimeters),
        ];
        for (src, expected_unit) in cases {
            let k = kinds(src);
            assert_eq!(
                k,
                vec![TokenKind::UnitLiteral {
                    value: 1.0,
                    unit: expected_unit.clone()
                }],
                "Failed for: {src}"
            );
        }
    }

    #[test]
    fn test_unit_suffix_boundary() {
        // "50.mliter" should NOT match "ml" because 'i' is an ID_CHAR
        let k = kinds("50.mliter");
        // Should be: 50 (int), . (dot), mliter (identifier)
        assert_eq!(k[0], TokenKind::IntLiteral(50));
        assert_eq!(k[1], TokenKind::Dot);
    }

    #[test]
    fn test_percent_literal() {
        assert_eq!(kinds("76%"), vec![TokenKind::PercentLiteral(76.0)]);
        assert_eq!(kinds("100%"), vec![TokenKind::PercentLiteral(100.0)]);
    }

    // -----------------------------------------------------------------------
    // Strings
    // -----------------------------------------------------------------------

    #[test]
    fn test_string_literal() {
        assert_eq!(
            kinds(r#""hello""#),
            vec![TokenKind::StringLiteral("hello".to_string())]
        );
    }

    #[test]
    fn test_string_escapes() {
        assert_eq!(
            kinds(r#""a\"b\\c""#),
            vec![TokenKind::StringLiteral("a\"b\\c".to_string())]
        );
        assert_eq!(
            kinds(r#""tab\there""#),
            vec![TokenKind::StringLiteral("tab\there".to_string())]
        );
    }

    #[test]
    fn test_string_unicode_escape() {
        assert_eq!(
            kinds(r#""\u0041""#),
            vec![TokenKind::StringLiteral("A".to_string())]
        );
    }

    #[test]
    fn test_unterminated_string() {
        let (tokens, errors) = lex(r#""unterminated"#);
        assert!(!errors.is_empty());
        assert!(matches!(tokens[0].kind, TokenKind::ErrorToken(_)));
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(
            kinds(r#""""#),
            vec![TokenKind::StringLiteral(String::new())]
        );
    }

    // -----------------------------------------------------------------------
    // Identifiers
    // -----------------------------------------------------------------------

    #[test]
    fn test_pascal_ident() {
        assert_eq!(
            kinds("Egg"),
            vec![TokenKind::PascalIdent("Egg".to_string())]
        );
        assert_eq!(
            kinds("FryingPan"),
            vec![TokenKind::PascalIdent("FryingPan".to_string())]
        );
        assert_eq!(
            kinds("A"),
            vec![TokenKind::PascalIdent("A".to_string())]
        );
    }

    #[test]
    fn test_snake_ident() {
        assert_eq!(
            kinds("egg"),
            vec![TokenKind::SnakeIdent("egg".to_string())]
        );
        assert_eq!(
            kinds("my_pan"),
            vec![TokenKind::SnakeIdent("my_pan".to_string())]
        );
        assert_eq!(
            kinds("oil_temp"),
            vec![TokenKind::SnakeIdent("oil_temp".to_string())]
        );
    }

    #[test]
    fn test_screaming_ident() {
        assert_eq!(
            kinds("MAX_TEMP"),
            vec![TokenKind::ScreamingIdent("MAX_TEMP".to_string())]
        );
        assert_eq!(
            kinds("DEFAULT_SERVINGS"),
            vec![TokenKind::ScreamingIdent("DEFAULT_SERVINGS".to_string())]
        );
        assert_eq!(
            kinds("ABC"),
            vec![TokenKind::ScreamingIdent("ABC".to_string())]
        );
    }

    #[test]
    fn test_two_char_uppercase_is_pascal() {
        // Two-char uppercase is PASCAL (SCREAM requires 3+)
        assert_eq!(
            kinds("PH"),
            vec![TokenKind::PascalIdent("PH".to_string())]
        );
    }

    #[test]
    fn test_keyword_boundary() {
        // "recipes" is NOT the keyword "recipe"
        assert_eq!(
            kinds("recipes"),
            vec![TokenKind::SnakeIdent("recipes".to_string())]
        );
        // But "recipe" alone IS the keyword
        assert_eq!(kinds("recipe"), vec![TokenKind::Recipe]);
    }

    // -----------------------------------------------------------------------
    // Keywords
    // -----------------------------------------------------------------------

    #[test]
    fn test_phase1_keywords() {
        assert_eq!(kinds("recipe"), vec![TokenKind::Recipe]);
        assert_eq!(kinds("ingredients"), vec![TokenKind::Ingredients]);
        assert_eq!(kinds("equipment"), vec![TokenKind::Equipment]);
        assert_eq!(kinds("steps"), vec![TokenKind::Steps]);
        assert_eq!(kinds("expected_result"), vec![TokenKind::ExpectedResult]);
        assert_eq!(kinds("nutrition"), vec![TokenKind::Nutrition]);
        assert_eq!(kinds("parallel"), vec![TokenKind::Parallel]);
        assert_eq!(kinds("import"), vec![TokenKind::Import]);
        assert_eq!(kinds("from"), vec![TokenKind::From]);
        assert_eq!(kinds("auto"), vec![TokenKind::Auto]);
    }

    #[test]
    fn test_bool_literals() {
        assert_eq!(kinds("true"), vec![TokenKind::BoolLiteral(true)]);
        assert_eq!(kinds("false"), vec![TokenKind::BoolLiteral(false)]);
    }

    // -----------------------------------------------------------------------
    // Comments
    // -----------------------------------------------------------------------

    #[test]
    fn test_regular_comment() {
        let k = kinds("// this is a comment");
        assert!(matches!(k[0], TokenKind::Comment(_)));
        if let TokenKind::Comment(c) = &k[0] {
            assert_eq!(c, " this is a comment");
        }
    }

    #[test]
    fn test_doc_comment() {
        let k = kinds("/// doc comment here");
        assert!(matches!(k[0], TokenKind::DocComment(_)));
        if let TokenKind::DocComment(c) = &k[0] {
            assert_eq!(c, " doc comment here");
        }
    }

    #[test]
    fn test_ai_hint() {
        let k = kinds("///ai: critical_for=food_safety");
        assert!(matches!(k[0], TokenKind::AiHint(_)));
        if let TokenKind::AiHint(c) = &k[0] {
            assert_eq!(c, " critical_for=food_safety");
        }
    }

    #[test]
    fn test_two_slash_ai_is_regular_comment() {
        // "//ai:" (two slashes) is a regular comment, NOT an AI hint
        let k = kinds("//ai: some hint");
        assert!(matches!(k[0], TokenKind::Comment(_)));
    }

    // -----------------------------------------------------------------------
    // Error recovery
    // -----------------------------------------------------------------------

    #[test]
    fn test_error_recovery_continues() {
        let (tokens, errors) = lex("42 ~ egg");
        // Should produce: IntLiteral(42), ErrorToken("~"), SnakeIdent("egg"), Eof
        assert_eq!(errors.len(), 1);
        let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
        assert_eq!(*kinds[0], TokenKind::IntLiteral(42));
        assert!(matches!(kinds[1], TokenKind::ErrorToken(_)));
        assert_eq!(*kinds[2], TokenKind::SnakeIdent("egg".to_string()));
        assert_eq!(*kinds[3], TokenKind::Eof);
    }

    // -----------------------------------------------------------------------
    // Span tracking
    // -----------------------------------------------------------------------

    #[test]
    fn test_span_tracking() {
        let (tokens, _) = lex("recipe FriedEgg");
        // "recipe" starts at line 1, col 1
        assert_eq!(tokens[0].span.start_line, 1);
        assert_eq!(tokens[0].span.start_col, 1);
        assert_eq!(tokens[0].span.byte_offset, 0);
        assert_eq!(tokens[0].span.byte_length, 6);
        // "FriedEgg" starts at col 8
        assert_eq!(tokens[1].span.start_col, 8);
    }

    #[test]
    fn test_multiline_span() {
        let (tokens, _) = lex("egg\noil");
        assert_eq!(tokens[0].span.start_line, 1); // egg on line 1
        assert_eq!(tokens[1].span.start_line, 2); // oil on line 2
    }

    // -----------------------------------------------------------------------
    // Integration: real fixture fragments
    // -----------------------------------------------------------------------

    #[test]
    fn test_ingredient_decl() {
        let k = kinds("egg: Egg(type: .Chicken, quantity: 1)");
        assert_eq!(k[0], TokenKind::SnakeIdent("egg".to_string()));
        assert_eq!(k[1], TokenKind::Colon);
        assert_eq!(k[2], TokenKind::PascalIdent("Egg".to_string()));
        assert_eq!(k[3], TokenKind::LeftParen);
        assert_eq!(k[4], TokenKind::SnakeIdent("type".to_string()));
        assert_eq!(k[5], TokenKind::Colon);
        assert_eq!(k[6], TokenKind::Dot);
        assert_eq!(k[7], TokenKind::PascalIdent("Chicken".to_string()));
        assert_eq!(k[8], TokenKind::Comma);
        assert_eq!(k[9], TokenKind::SnakeIdent("quantity".to_string()));
        assert_eq!(k[10], TokenKind::Colon);
        assert_eq!(k[11], TokenKind::IntLiteral(1));
        assert_eq!(k[12], TokenKind::RightParen);
    }

    #[test]
    fn test_process_call() {
        let k = kinds("Heat(pan, to: 180.celsius, using: stove)");
        assert_eq!(k[0], TokenKind::PascalIdent("Heat".to_string()));
        assert_eq!(k[1], TokenKind::LeftParen);
        assert_eq!(k[2], TokenKind::SnakeIdent("pan".to_string()));
        assert_eq!(k[3], TokenKind::Comma);
        assert_eq!(k[4], TokenKind::SnakeIdent("to".to_string()));
        assert_eq!(k[5], TokenKind::Colon);
        assert_eq!(
            k[6],
            TokenKind::UnitLiteral {
                value: 180.0,
                unit: Unit::Celsius
            }
        );
        assert_eq!(k[7], TokenKind::Comma);
        assert_eq!(k[8], TokenKind::SnakeIdent("using".to_string()));
        assert_eq!(k[9], TokenKind::Colon);
        assert_eq!(k[10], TokenKind::SnakeIdent("stove".to_string()));
        assert_eq!(k[11], TokenKind::RightParen);
    }

    #[test]
    fn test_comparison_expr() {
        let k = kinds("oil.state.temperature >= 170.celsius");
        assert_eq!(k[0], TokenKind::SnakeIdent("oil".to_string()));
        assert_eq!(k[1], TokenKind::Dot);
        assert_eq!(k[2], TokenKind::SnakeIdent("state".to_string()));
        assert_eq!(k[3], TokenKind::Dot);
        assert_eq!(k[4], TokenKind::SnakeIdent("temperature".to_string()));
        assert_eq!(k[5], TokenKind::GreaterEqual);
        assert_eq!(
            k[6],
            TokenKind::UnitLiteral {
                value: 170.0,
                unit: Unit::Celsius
            }
        );
    }

    #[test]
    fn test_destructure() {
        let k = kinds("Crack(egg) -> [yolk, white]");
        assert_eq!(k[0], TokenKind::PascalIdent("Crack".to_string()));
        assert_eq!(k[1], TokenKind::LeftParen);
        assert_eq!(k[2], TokenKind::SnakeIdent("egg".to_string()));
        assert_eq!(k[3], TokenKind::RightParen);
        assert_eq!(k[4], TokenKind::Arrow);
        assert_eq!(k[5], TokenKind::LeftBracket);
        assert_eq!(k[6], TokenKind::SnakeIdent("yolk".to_string()));
        assert_eq!(k[7], TokenKind::Comma);
        assert_eq!(k[8], TokenKind::SnakeIdent("white".to_string()));
        assert_eq!(k[9], TokenKind::RightBracket);
    }

    #[test]
    fn test_annotation() {
        let k = kinds("@version(\"1.0.0\")");
        assert_eq!(k[0], TokenKind::At);
        assert_eq!(k[1], TokenKind::SnakeIdent("version".to_string()));
        assert_eq!(k[2], TokenKind::LeftParen);
        assert_eq!(k[3], TokenKind::StringLiteral("1.0.0".to_string()));
        assert_eq!(k[4], TokenKind::RightParen);
    }

    #[test]
    fn test_step_with_number() {
        let k = kinds("1: Heat(pan, to: 180.celsius)");
        assert_eq!(k[0], TokenKind::IntLiteral(1));
        assert_eq!(k[1], TokenKind::Colon);
        assert_eq!(k[2], TokenKind::PascalIdent("Heat".to_string()));
    }

    #[test]
    fn test_enum_path() {
        let k = kinds("Doneness.MediumRare");
        assert_eq!(k[0], TokenKind::PascalIdent("Doneness".to_string()));
        assert_eq!(k[1], TokenKind::Dot);
        assert_eq!(k[2], TokenKind::PascalIdent("MediumRare".to_string()));
    }

    #[test]
    fn test_expected_result_keyword() {
        let k = kinds("expected_result: FriedEgg {}");
        assert_eq!(k[0], TokenKind::ExpectedResult);
        assert_eq!(k[1], TokenKind::Colon);
        assert_eq!(k[2], TokenKind::PascalIdent("FriedEgg".to_string()));
        assert_eq!(k[3], TokenKind::LeftBrace);
        assert_eq!(k[4], TokenKind::RightBrace);
    }

    #[test]
    fn test_nutrition_auto() {
        let k = kinds("nutrition: auto");
        assert_eq!(k[0], TokenKind::Nutrition);
        assert_eq!(k[1], TokenKind::Colon);
        assert_eq!(k[2], TokenKind::Auto);
    }

    // ===================================================================
    // Fixture snapshot tests
    //
    // These tests tokenize full .saffron files end-to-end. We verify:
    //   - Zero lexer errors for valid files
    //   - Correct total token count (excluding Eof)
    //   - Key token sequences at strategic positions
    //   - Last token is always Eof
    // ===================================================================

    /// Helper: lex a file from the fixtures directory.
    /// `rel_path` is relative to the workspace root (e.g. "tests/fixtures/...").
    fn lex_fixture(rel_path: &str) -> (Vec<Token>, Vec<LexError>) {
        // CARGO_MANIFEST_DIR points to crates/saffron-lexer/.
        // Fixtures live at the workspace root, two levels up.
        let manifest = env!("CARGO_MANIFEST_DIR");
        let workspace_root = std::path::Path::new(manifest)
            .parent()  // crates/
            .and_then(|p| p.parent())  // workspace root
            .expect("Cannot find workspace root from CARGO_MANIFEST_DIR");
        let full_path = workspace_root.join(rel_path);
        let source = std::fs::read_to_string(&full_path)
            .unwrap_or_else(|e| panic!("Failed to read fixture {}: {e}", full_path.display()));
        Lexer::new(&source, rel_path).tokenize()
    }

    // -----------------------------------------------------------------------
    // Valid fixtures — must produce 0 errors
    // -----------------------------------------------------------------------

    #[test]
    fn fixture_fried_egg_no_errors() {
        let (tokens, errors) = lex_fixture(
            "tests/fixtures/valid/basic/fried_egg.saffron",
        );
        assert!(
            errors.is_empty(),
            "fried_egg.saffron produced {} errors: {:?}",
            errors.len(),
            errors
        );
        // Last token must be Eof
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
        // Sanity: should have a significant number of tokens
        let non_eof: Vec<_> = tokens
            .iter()
            .filter(|t| t.kind != TokenKind::Eof)
            .collect();
        assert!(
            non_eof.len() > 100,
            "Expected >100 tokens, got {}",
            non_eof.len()
        );
    }

    #[test]
    fn fixture_fried_egg_key_tokens() {
        let (tokens, _) = lex_fixture(
            "tests/fixtures/valid/basic/fried_egg.saffron",
        );
        let k: Vec<&TokenKind> = tokens
            .iter()
            .filter(|t| !matches!(t.kind, TokenKind::Eof | TokenKind::Comment(_)))
            .map(|t| &t.kind)
            .collect();

        // "recipe FriedEgg {"
        assert_eq!(*k[0], TokenKind::Recipe);
        assert_eq!(*k[1], TokenKind::PascalIdent("FriedEgg".to_string()));
        assert_eq!(*k[2], TokenKind::LeftBrace);

        // Verify annotations are present: @version("1.0.0")
        assert_eq!(*k[3], TokenKind::At);
        assert_eq!(*k[4], TokenKind::SnakeIdent("version".to_string()));
        assert_eq!(*k[5], TokenKind::LeftParen);
        assert_eq!(*k[6], TokenKind::StringLiteral("1.0.0".to_string()));

        // Find "ingredients" keyword somewhere in the stream
        assert!(
            k.iter().any(|t| **t == TokenKind::Ingredients),
            "Missing 'ingredients' keyword"
        );

        // Find "equipment" keyword
        assert!(
            k.iter().any(|t| **t == TokenKind::Equipment),
            "Missing 'equipment' keyword"
        );

        // Find "steps" keyword
        assert!(
            k.iter().any(|t| **t == TokenKind::Steps),
            "Missing 'steps' keyword"
        );

        // Find unit literal: 180.celsius
        assert!(
            k.iter().any(|t| **t
                == TokenKind::UnitLiteral {
                    value: 180.0,
                    unit: Unit::Celsius
                }),
            "Missing 180.celsius unit literal"
        );

        // Find unit literal: 50.ml
        assert!(
            k.iter().any(|t| **t
                == TokenKind::UnitLiteral {
                    value: 50.0,
                    unit: Unit::Milliliters
                }),
            "Missing 50.ml unit literal"
        );

        // Find "nutrition" keyword
        assert!(
            k.iter().any(|t| **t == TokenKind::Nutrition),
            "Missing 'nutrition' keyword"
        );

        // Find "auto" keyword (nutrition: auto)
        assert!(
            k.iter().any(|t| **t == TokenKind::Auto),
            "Missing 'auto' keyword"
        );

        // Find Arrow token (->) from "Crack(egg) -> [yolk, white]"
        assert!(
            k.iter().any(|t| **t == TokenKind::Arrow),
            "Missing '->' arrow token"
        );
    }

    #[test]
    fn fixture_grilled_steak_no_errors() {
        let (tokens, errors) = lex_fixture(
            "tests/fixtures/valid/basic/grilled_steak.saffron",
        );
        assert!(
            errors.is_empty(),
            "grilled_steak.saffron produced {} errors: {:?}",
            errors.len(),
            errors
        );
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);

        let k: Vec<&TokenKind> = tokens.iter().map(|t| &t.kind).collect();

        // Verify recipe name
        assert!(k.contains(&&TokenKind::Recipe));
        assert!(k.contains(&&TokenKind::PascalIdent("GrilledSteak".to_string())));

        // Verify specific unit literals
        assert!(
            k.contains(&&TokenKind::UnitLiteral {
                value: 300.0,
                unit: Unit::Grams
            }),
            "Missing 300.grams"
        );
        assert!(
            k.contains(&&TokenKind::UnitLiteral {
                value: 230.0,
                unit: Unit::Celsius
            }),
            "Missing 230.celsius"
        );
        assert!(
            k.contains(&&TokenKind::UnitLiteral {
                value: 2.5,
                unit: Unit::Centimeters
            }),
            "Missing 2.5.cm"
        );
        assert!(
            k.contains(&&TokenKind::UnitLiteral {
                value: 57.0,
                unit: Unit::Celsius
            }),
            "Missing 57.celsius"
        );
    }

    #[test]
    fn fixture_boiled_pasta_no_errors() {
        let (tokens, errors) = lex_fixture(
            "tests/fixtures/valid/basic/boiled_pasta.saffron",
        );
        assert!(
            errors.is_empty(),
            "boiled_pasta.saffron produced {} errors: {:?}",
            errors.len(),
            errors
        );
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);

        let k: Vec<&TokenKind> = tokens.iter().map(|t| &t.kind).collect();

        assert!(k.contains(&&TokenKind::PascalIdent("BoiledPasta".to_string())));

        // "water.state.phase == Phase.Liquid" — should contain Equal (==)
        assert!(k.contains(&&TokenKind::Equal), "Missing '==' operator");

        // 2.liters — float unit literal
        assert!(
            k.contains(&&TokenKind::UnitLiteral {
                value: 2.0,
                unit: Unit::Liters
            }),
            "Missing 2.liters"
        );

        // 9.minutes
        assert!(
            k.contains(&&TokenKind::UnitLiteral {
                value: 9.0,
                unit: Unit::Minutes
            }),
            "Missing 9.minutes"
        );

        // Arrow from drain destructure
        assert!(k.contains(&&TokenKind::Arrow), "Missing '->' token");
    }

    // -----------------------------------------------------------------------
    // Invalid fixtures — should also lex without errors!
    // (Lexer doesn't validate semantics; these are syntactically valid)
    // -----------------------------------------------------------------------

    #[test]
    fn fixture_fry_water_lexes_cleanly() {
        let (tokens, errors) = lex_fixture(
            "tests/fixtures/invalid/type_errors/fry_water.saffron",
        );
        // Type errors are semantic, not lexical — lexer should succeed
        assert!(
            errors.is_empty(),
            "fry_water.saffron produced {} lex errors: {:?}",
            errors.len(),
            errors
        );
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);

        let k: Vec<&TokenKind> = tokens.iter().map(|t| &t.kind).collect();
        assert!(k.contains(&&TokenKind::PascalIdent("FryWater".to_string())));
        assert!(
            k.contains(&&TokenKind::UnitLiteral {
                value: 200.0,
                unit: Unit::Milliliters
            }),
            "Missing 200.ml"
        );
    }

    #[test]
    fn fixture_temp_mismatch_lexes_cleanly() {
        let (tokens, errors) = lex_fixture(
            "tests/fixtures/invalid/type_errors/temp_mismatch.saffron",
        );
        assert!(
            errors.is_empty(),
            "temp_mismatch.saffron produced {} lex errors: {:?}",
            errors.len(),
            errors
        );
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);

        let k: Vec<&TokenKind> = tokens.iter().map(|t| &t.kind).collect();
        assert!(k.contains(&&TokenKind::PascalIdent("TempMismatch".to_string())));

        // Both unit systems present
        assert!(
            k.contains(&&TokenKind::UnitLiteral {
                value: 356.0,
                unit: Unit::Fahrenheit
            }),
            "Missing 356.fahrenheit"
        );
        assert!(
            k.contains(&&TokenKind::UnitLiteral {
                value: 180.0,
                unit: Unit::Celsius
            }),
            "Missing 180.celsius"
        );

        // GreaterEqual from the comparison
        assert!(k.contains(&&TokenKind::GreaterEqual), "Missing '>=' operator");
    }
}
