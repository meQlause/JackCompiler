mod errors;
mod prelude;
mod utils;
use prelude::*;

fn main() {
    let mut compiler = CompilationEngine::new(String::from("Ball.jack"));
    compiler.compile();
}
