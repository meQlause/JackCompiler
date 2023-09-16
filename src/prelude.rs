pub use core::panic;
pub use std::collections::HashMap;
pub use std::default::Default;
pub use std::fmt::{Debug, Display, Formatter, Result};
pub use std::fs::File;
pub use std::io::Write;
pub use std::io::{BufRead, BufReader};
pub use std::sync::Arc;
pub use std::vec;


pub use crate::errors::ParsingError;
pub use crate::utils::compiler_engine::compilation_engine::CompilationEngine;
pub use crate::utils::compiler_engine::grammar::Grammar;
pub use crate::utils::compiler_engine::grammar_trait::{
    Expressions, OptionalExpressions, ProgramStructure, Statements,
};
pub use crate::utils::compiler_engine::stack_compiler::StackCompiler;
pub use crate::utils::jack_tokenizer::elements::{Elements, TokenKind};
pub use crate::utils::jack_tokenizer::tokenizer::{JackTokenizer, Tokenizer};

pub type ResultParser<T> = std::result::Result<T, ParsingError>;
pub type CompilerFunc = dyn FnMut(&mut File, &mut JackTokenizer, &mut i8) -> String;

