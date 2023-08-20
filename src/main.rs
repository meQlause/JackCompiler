mod utils;
use utils::parser::CompilationEngine;
use utils::jack_tokenizer::JackTokenizer;

fn main() {
    let mut tokenizer = JackTokenizer::new("Main.jack");
    loop {
        if !tokenizer.has_more_token() {
            break;
        }
        match &tokenizer.string_val{
            Some(a) => println!("{}", a),
            None => continue,
        }
        // match tokenizer.symbol {
        //     Some(a) => println!("{}", a),
        //     None => continue,
        // }
    }
}
