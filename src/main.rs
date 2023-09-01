mod utils;
use std::fs::File;
use utils::jack_tokenizer::JackTokenizer;
use utils::parser::CompilationEngine;

fn main() {
    let mut parser = CompilationEngine::new(File::create("Main.xml").unwrap());
    let mut tokenizer = JackTokenizer::new("Main.jack");

    loop {
        if !tokenizer.has_more_token() {
            println!("{:?}", tokenizer.symbol);
            parser.compile(&mut tokenizer);
            break;
        }
            println!("{:?}", tokenizer.symbol);

        parser.compile(&mut tokenizer);
    }
    // match tokenizer.symbol {
    //     Some(a) => println!("{}", a),
    //     None => continue,
    // }
}
