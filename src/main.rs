mod prelude;
mod utils;
mod errors;
use prelude::*;

fn main() {
    // let mut parser = CompilationEngine::new(File::create("Main.xml").unwrap());

    let mut tokenizer = JackTokenizer::new("Main.jack");
    while let true = tokenizer.has_more_token(30) {
        println!("{:?}", tokenizer.tokens);
        println!("{:?}", tokenizer.token_kinds);
    }
    while let true = tokenizer.has_more_token(3) {
        println!("{:?}", tokenizer.tokens);
        println!("{:?}", tokenizer.token_kinds);
    }
    while let true = tokenizer.has_more_token(1) {
        println!("{:?}", tokenizer.tokens);
        println!("{:?}", tokenizer.token_kinds);
    }
}
