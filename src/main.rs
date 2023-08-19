mod utils;
use utils::jack_tokenizer::JackTokenizer;

fn main() {
    let mut tokenizer = JackTokenizer::new("Main.jack");
    tokenizer.has_more_token();
}
