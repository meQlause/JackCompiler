use crate::prelude::*;
use std::panic;

#[derive(Debug)]
pub struct JackTokenizer {
    file_name: String,
    line_string: String,
    elements: Elements,
    pub current_line_position: usize,
    pub current_char_position: usize,
    pub token_kind: TokenKind,
    pub token: String,
}
trait TokenizerPrivate {
    fn get_line_string(&mut self) -> Option<String>;
    fn get_token(&mut self) -> Option<String>;
    fn set_token_kind(&mut self, token: &str) -> TokenKind;
}
pub trait Tokenizer {
    fn new(file_name: &str) -> Self;
    fn has_more_token(&mut self) -> bool;
    fn get_context(&mut self, context: usize) -> String;
}
impl Tokenizer for JackTokenizer {
    fn new(file_name: &str) -> Self {
        Self {
            file_name: file_name.to_string(),
            current_line_position: 0usize,
            current_char_position: 0usize,
            elements: Elements::default(),
            line_string: String::new(),
            token: String::new(),
            token_kind: TokenKind::None,
        }
    }

    fn get_context(&mut self, context: usize) -> String {
        let (temp_line, temp_char, mut temp) = (
            self.current_line_position.to_owned(),
            self.current_char_position.to_owned(),
            Vec::new(),
        );
        if context == 1 {
            if let Some(token) = self.get_token() {
                temp.push(token);
            } else {
                panic!("No Token Available");
            }
        } else {
            for _ in 0..context {
                if let Some(token) = self.get_token() {
                    temp.push(token);
                } else {
                    panic!("No Token Available");
                }
            }
        }
        self.current_line_position = temp_line - 1;
        self.current_char_position = temp_char;
        self.line_string = self.get_line_string().unwrap_or(String::new());
        temp.last().cloned().unwrap()
    }

    fn has_more_token(&mut self) -> bool {
        if let Some(token) = self.get_token() {
            let token_kind = self.set_token_kind(&token);
            self.token_kind = token_kind;
            self.token = token;
            true
        } else {
            false
        }
    }
}

impl TokenizerPrivate for JackTokenizer {
    fn get_line_string(&mut self) -> Option<String> {
        let file_name = self.file_name.clone();
        for (count, line) in BufReader::new(File::open(file_name).expect("Can't Read The File"))
            .lines()
            .enumerate()
        {
            let line_to_read = line.unwrap();
            if line_to_read.trim().is_empty()
                || line_to_read.trim().starts_with('/')
                || line_to_read.trim().starts_with('*')
            {
                continue;
            }
            if count >= self.current_line_position {
                self.current_line_position = count + 1;
                return Some(line_to_read.trim().to_string());
            }
        }
        None
    }

    fn get_token(&mut self) -> Option<String> {
        loop {
            let (mut syntax, mut is_string, mut is_negative) = (String::new(), false, false);
            if self.current_char_position == 0
                || self.current_char_position == self.line_string.len()
            {
                self.line_string = self.get_line_string()?;
                self.current_char_position = 0;
            }
            for character in self.line_string.chars().skip(self.current_char_position) {
                self.current_char_position += 1;
                // In-code Comment Handling
                if character == '/'
                    && self.line_string.chars().nth(self.current_char_position) == Some('/')
                {
                    self.current_char_position = 0;
                    break;
                }

                // String Handling
                if character == '"' || is_string {
                    if syntax.contains('"') && character == '"' {
                        is_string = false;
                        syntax.push(character);
                        continue;
                    }
                    is_string = true;
                    syntax.push(character);
                    continue;
                }

                // Character Break With Space Handling
                if character == ' ' {
                    syntax = syntax.trim().to_string();
                    if syntax.is_empty() {
                        continue;
                    }
                    return Some(syntax);
                }

                // handle negative number
                if character == '-' || is_negative {
                    is_negative = true;
                    if String::from(character).parse::<i32>().is_ok() {
                        syntax.push(character);
                        continue;
                    } else if character != '-' {
                        self.current_char_position -= 1;
                        return Some(syntax);
                    }
                    syntax.push(character);
                    continue;
                }

                // Character Break With symbols Handling
                if self.elements.contains_symbol(&character.to_string()) {
                    syntax = syntax.trim().to_string();
                    if syntax.is_empty() {
                        return Some(character.to_string());
                    }
                    self.current_char_position -= 1;
                    return Some(syntax);
                }
                syntax.push(character);
            }
        }
    }

    fn set_token_kind(&mut self, token: &str) -> TokenKind {
        if self.elements.contains_keyword(token) {
            return TokenKind::Keyword;
        }
        if self.elements.contains_symbol(token) {
            return TokenKind::Symbol;
        }
        if token.parse::<i128>().is_ok() {
            return TokenKind::IntVal;
        }
        if token.starts_with('"') {
            return TokenKind::StringVal;
        }
        TokenKind::Identifier
    }
}
