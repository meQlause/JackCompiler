use std::fs::File;
use std::io::{BufReader, BufRead};
use std::sync::Arc;
pub struct TokenTypes<String> (
    String,
);
impl TokenTypes<String> {
    pub fn set(r: String) -> TokenTypes<String> {
        Self(r)
    }

}
pub struct JackTokenizer {
    file: File,
    line: i128,
    symbols: Arc<[char; 19]>,
    pub keyword: Option<TokenTypes<String>>,
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
            line: 0,
            symbols: Arc::new(['{', '}', '(', ')', '[', ']', '.', ',', ';', '+', '-', '*', '/', '&', '|', '<', '>', '=', '~']),
            keyword: None, 
            symbol: None, 
            identifier: None, 
            int_val: None, 
            string_val: None 
        }
    }

    pub fn has_more_token(&mut self) -> bool {
        for lines in BufReader::new(&self.file).lines() {
            self.line += 1;
            let line_to_read = lines.expect("Can't read line");
            let mut syntax :String = String::new();
            if line_to_read.trim().as_bytes().len() == 0 || line_to_read.chars().next() == Some('/') {continue;}
            for c in line_to_read.chars() {
                if self.symbols.contains(&c) || c == ' ' {
                    if c != ' ' {
                        if syntax.trim().as_bytes().len() != 0 {
                            println!("{} is a syntax", syntax.trim());
                            syntax.clear();
                        }
                        println!("{} is a symbol", c);
                        continue;
                    }
                }
                syntax.push(c);
                
            }
        }
        true
    }

}
