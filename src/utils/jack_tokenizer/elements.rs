use crate::prelude::*;

#[derive(Debug)]
pub enum TokenKind {
    Keyword,
    Symbol,
    Identifier,
    IntVal,
    StringVal,
    None,
}
impl PartialEq for TokenKind {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (TokenKind::Keyword, TokenKind::Keyword)
                | (TokenKind::Symbol, TokenKind::Symbol)
                | (TokenKind::Identifier, TokenKind::Identifier)
                | (TokenKind::IntVal, TokenKind::IntVal)
                | (TokenKind::StringVal, TokenKind::StringVal)
        )
    }
}
#[derive(Debug)]
pub struct Elements {
    symbols: Arc<[&'static str; 19]>,
    keywords: Arc<[&'static str; 21]>,
}
impl Default for Elements {
    fn default() -> Self {
        Self {
            symbols: Arc::new([
                "{", "}", "(", ")", "[", "]", ".", ",", ";", "+", "-", "*", "/", "&", "|", "<",
                ">", "=", "~",
            ]),
            keywords: Arc::new([
                "class",
                "constructor",
                "function",
                "method",
                "field",
                "static",
                "var",
                "int",
                "char",
                "boolean",
                "void",
                "true",
                "false",
                "null",
                "this",
                "let",
                "do",
                "if",
                "else",
                "while",
                "return",
            ]),
        }
    }
}

impl Elements {
    pub fn contains_symbol(&mut self, symbol: &str) -> bool {
        self.symbols.contains(&symbol)
    }
    pub fn contains_keyword(&mut self, syntax: &str) -> bool {
        self.keywords.contains(&syntax)
    }
}
