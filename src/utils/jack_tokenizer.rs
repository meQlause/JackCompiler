use std::fs::File;
use std::io::{BufReader, BufRead};
use std::sync::Arc;
use std::collections::HashMap;

pub struct JackTokenizer {
    file: File,
    line: i128,
    total_line: i128,
    token_maks: usize,
    current_token: usize,
    symbols: Arc<[String; 19]>,
    keywords: Arc<[String; 21]>,
    tokens: HashMap<i128, Vec<String>>,
    pub keyword: Option<String>,
    pub symbol: Option<char>,
    pub identifier: Option<String>,
    pub int_val: Option<i128>,
    pub string_val: Option<String>,
}

impl JackTokenizer {
    pub fn new(file_name: &str)  -> Self { 
        let file = File::open(file_name).expect("Error opening file");
        Self { 
            file: file, 
            line: -1,
            total_line: 0,
            token_maks: 0usize,
            current_token: 1usize,
            symbols: Arc::new(["{".to_string(), "}".to_string(), "(".to_string(), ")".to_string(), "[".to_string(), "]".to_string(), ".".to_string(), ",".to_string(), ";".to_string(), "+".to_string(), "-".to_string(), "*".to_string(), "/".to_string(), "&".to_string(), "|".to_string(), "<".to_string(), ">".to_string(), "=".to_string(), "~".to_string()]),
            keywords: Arc::new(["class".to_string(), "constructor".to_string(), "function".to_string(), "method".to_string(), "field".to_string(), "static".to_string(), "var".to_string(), "int".to_string(), "char".to_string(), "boolean".to_string(), "void".to_string(), "true".to_string(), "false".to_string(), "null".to_string(), "this".to_string(), "let".to_string(), "do".to_string(), "if".to_string(), "else".to_string(), "while".to_string(), "return".to_string()]),
            tokens: HashMap::new(),
            keyword: None, 
            symbol: None, 
            identifier: None, 
            int_val: None, 
            string_val: None 
        }
    }

    fn tokenizer(&mut self) {
        for lines in BufReader::new(&self.file).lines() {
            let (line_to_read, mut syntax, mut is_string) = (lines.expect("Can't read line"), String::new(), false);
            self.total_line += 1;
            if line_to_read.trim().as_bytes().len() == 0 || line_to_read.chars().next() == Some('/') {continue;}
            self.tokens.insert(self.total_line, Vec::new());
            for c in line_to_read.chars() {
                if c == '"' || is_string {
                    if syntax.contains('"') && c == '"' {
                        syntax.push(c);
                        is_string = false;
                        continue;
                    }
                    syntax.push(c);
                    is_string = true;
                    continue;
                }
                if c == ' ' {
                    syntax = syntax.trim().to_string();
                    if !syntax.is_empty() {
                        let value = self.tokens.get_mut(&self.total_line).expect("Invalid key"); 
                        value.push(syntax.to_string());
                        syntax.clear();
                    }
                    continue;
                    }
                if self.symbols.contains(&c.to_string()) {
                    syntax = syntax.trim().to_string();
                    if !syntax.is_empty() {
                        let value = self.tokens.get_mut(&self.total_line).expect("Invalid key"); 
                        value.push(syntax.to_string());
                        syntax.clear();
                    } 
                    let value = self.tokens.get_mut(&self.total_line).expect("Invalid key"); 
                    value.push(c.to_string());
                    continue;
                    }
                syntax.push(c);
                }
            }
            dbg!("{:?}", &self.tokens); 
        }
    

    fn get_position(&mut self) -> bool {
        if self.current_token >= self.token_maks { 
            loop {
                self.line += 1;
                if self.line > self.total_line { return false; }
                if let Some(list_token) = self.tokens.get(&self.line) { 
                    self.token_maks = list_token.len() - 1;
                    self.current_token = 0;
                    return true 
                }
            }
        } else {
            self.current_token += 1;
            return true;
        }
    }
    pub fn has_more_token(&mut self) -> bool {
        if self.tokens.len() < 1 {self.tokenizer()}
        if self.get_position() {
            self.advance();
            return true;
        }
        return false;
    }

    fn advance(&mut self) {
        let list_token = self.tokens.get(&self.line).unwrap();
        if self.keywords.contains(&list_token[self.current_token]) {
            self.keyword = Some(list_token[self.current_token].to_string()); 
            self.symbol = None;
            self.identifier = None; 
            self.int_val = None;
            self.string_val = None; 
            return;
        }
        if self.symbols.contains(&list_token[self.current_token]) {
            self.keyword = None; 
            self.symbol = list_token[self.current_token].chars().next();
            self.identifier = None; 
            self.int_val = None;
            self.string_val = None;
            return;
        }
        match list_token[self.current_token].parse::<i128>() {
            Ok(a) => {
                self.keyword = None; 
                self.symbol = None; 
                self.identifier = None; 
                self.int_val = Some(a);
                self.string_val = None;
                return;
            },
            Err(_) => {
                if list_token[self.current_token].chars().next() == Some('"') {
                    self.keyword = None; 
                    self.symbol = None; 
                    self.identifier = None; 
                    self.int_val = None;
                    self.string_val = Some(list_token[self.current_token].to_string());
                    return;
                }
                    self.keyword = None; 
                    self.symbol = None; 
                    self.identifier = Some(list_token[self.current_token].to_string()); 
                    self.int_val = None;
                    self.string_val = None;
                    return;
            }
        }
    }
}

