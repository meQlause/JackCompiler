mod utils;
use utils::parser::CompilationEngine;
use utils::jack_tokenizer::JackTokenizer;
use std::fs::File;

fn main() {
    let mut parser = CompilationEngine::new(File::create("Main.xml").unwrap());
    let mut tokenizer = JackTokenizer::new("Main.jack");

    loop {
        if !tokenizer.has_more_token() { 
            parser.compile(&tokenizer);
            break; 
        }
        parser.compile(&tokenizer);
    }
        // match tokenizer.symbol {
        //     Some(a) => println!("{}", a),
        //     None => continue,
        // }
}

