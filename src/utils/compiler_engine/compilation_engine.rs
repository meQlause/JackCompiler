use std::panic;

use crate::prelude::*;

use super::grammar_trait::OptionalExpressions;

#[derive(Debug)]
pub struct CompilationEngine {
    file: File,
    stack: StackCompiler,
    tokenizer: JackTokenizer,
}

impl CompilationEngine {
    pub fn new(file_input: String) -> CompilationEngine {
        CompilationEngine {
            file: File::create("Main.xml").unwrap(),
            stack: StackCompiler::default(),
            tokenizer: JackTokenizer::new(&file_input),
        }
    }
    fn matching(file: &mut File, tag: String, token: String) -> String {
        if tag == *"Fn" || tag == *"Pop" || tag == *"Iterate" {
            token
        } else {
            let to_write = Self::parse(&tag, &token);
            writeln!(file, "{to_write}").unwrap();
            "Done".to_string()
        }
    }

    fn parse(tag: &String, token: &String) -> String {
        format!("<{tag}>{token}</{tag}>")
    }

    pub fn class_compiler(
        file: &mut File,
        tokenizer: &mut JackTokenizer,
        state: &mut i8,
    ) -> String {
        if ["static", "field"].contains(&tokenizer.token.as_str()) {
            *state = 4;
        }
        if ["constructor", "function", "method"].contains(&tokenizer.token.as_str()) {
            *state = 5;
        }
        match Grammar::class_grammar(&tokenizer.token, &tokenizer.token_kind, state) {
            Ok((token, tag)) => {
                if *state == 1 {
                    writeln!(file, "<class>").unwrap();
                }
                if *state == 0 {
                    writeln!(file, "</class>").unwrap();
                    return "Done".to_string();
                }
                *state += 1;
                Self::matching(file, tag, token)
            }
            Err(error) => {
                panic!("{error}");
            }
        }
    }

    pub fn return_compiler(
        file: &mut File,
        tokenizer: &mut JackTokenizer,
        state: &mut i8,
    ) -> String {
        match Grammar::return_statement_grammar(&tokenizer.token, &tokenizer.token_kind, state) {
            Ok((token, tag)) => {
                if *state == 1 {
                    writeln!(file, "<return>").unwrap();
                } else if *state == 0 {
                    writeln!(file, "</return>").unwrap();
                }
                *state += 1;
                Self::matching(file, tag, token)
            }
            Err(error) => {
                panic!("{error}");
            }
        }
    }

    #[rustfmt::skip]
    pub fn do_compiler(
        file: &mut File,
        tokenizer: &mut JackTokenizer,
        state: &mut i8,
    ) -> String {
        match Grammar::do_statement_grammar(&tokenizer.token, &tokenizer.token_kind, state) {
            Ok((token, tag)) => {
                if *state == 1 {
                    writeln!(file, "<do>").unwrap();
                } else if *state == 0 {
                    writeln!(file, "</do>").unwrap();
                }
                *state += 1;
                Self::matching(file, tag, token)
            }
            Err(error) => {
                panic!("{error}");
            }
        }
    }

    pub fn expression_compiler(
        file: &mut File,
        tokenizer: &mut JackTokenizer,
        state: &mut i8,
    ) -> String {
        if *state == 3 && ["+", "-", "*", "/", "&", "|", "<", ">", "="].contains(&tokenizer.token.as_str()) && tokenizer.token_kind == TokenKind::Symbol {
            Self::matching(file, "symbol".to_string(), tokenizer.token.to_string());
            *state = 2;
            return "Done".to_string();
        }  
        match Grammar::expression_grammar(&tokenizer.token, &tokenizer.token_kind, state) {
            Ok((token, tag)) => {
                if *state == 1 {
                    writeln!(file, "<expression>").unwrap();
                } else if *state == 0 {
                    writeln!(file, "</expression>").unwrap();
                }
                *state += 1;
                Self::matching(file, tag, token)
            }
            Err(error) => {
                panic!("{error}");
            }
        }
    }

    pub fn var_dec_compiler(
        file: &mut File,
        tokenizer: &mut JackTokenizer,
        state: &mut i8,
    ) -> String {
        if *state == 4 && tokenizer.token == "," && tokenizer.token_kind == TokenKind::Symbol  {
            Self::matching(file, "symbol".to_string(), tokenizer.token.to_string());
            *state = 3;
            return "Done".to_string();
        }
        match Grammar::var_dec_grammar(&tokenizer.token, &tokenizer.token_kind, state) {
            Ok((token, tag)) => {
                if *state == 1 {
                    writeln!(file, "<varDec>").unwrap();
                } else if *state == 0 {
                    writeln!(file, "</varDec>").unwrap();
                }
                *state += 1;
                Self::matching(file, tag, token)
            }
            Err(error) => {
                panic!("{error}");
            }
        }
    }

    pub fn expression_list_compiler(
        file: &mut File,
        tokenizer: &mut JackTokenizer,
        state: &mut i8,
    ) -> String {
        if tokenizer.token == ")" && *state == 1 {
            writeln!(file, "<expressionList>").unwrap();
            writeln!(file, "</expressionList>").unwrap();
            return String::from("SafePop");
        }
        if *state == 3 && tokenizer.token == "," && tokenizer.token_kind == TokenKind::Symbol  {
            Self::matching(file, "symbol".to_string(), tokenizer.token.to_string());
            *state = 2;
            return "Done".to_string();
        } 
        match Grammar::expression_list_grammar(&tokenizer.token, &tokenizer.token_kind, state) {
            Ok((token, tag)) => {
                if *state == 1 {
                    writeln!(file, "<expressionList>").unwrap();
                } else if *state == 0 {
                    writeln!(file, "</expressionList>").unwrap();
                }
                *state += 1;
                Self::matching(file, tag, token)
            }
            Err(error) => {
                panic!("{error}");
            }
        }
    }

    pub fn subroutine_body_compiler(
        file: &mut File,
        tokenizer: &mut JackTokenizer,
        state: &mut i8,
    ) -> String {
        if tokenizer.token == "var" {
            *state = 2;
        }
        match Grammar::subroutine_body_grammar(&tokenizer.token, &tokenizer.token_kind, state) {
            Ok((token, tag)) => {
                if *state == 1 {
                    writeln!(file, "<subroutineBody>").unwrap();
                } else if *state == 0 {
                    writeln!(file, "</subroutineBody>").unwrap();
                }
                *state += 1;
                Self::matching(file, tag, token)
            }
            Err(error) => {
                panic!("{error}");
            }
        }
    }

    pub fn class_var_dec_compiler(
        file: &mut File,
        tokenizer: &mut JackTokenizer,
        state: &mut i8,
    ) -> String {
        if *state == 4 && tokenizer.token == "," && tokenizer.token_kind == TokenKind::Symbol  {
            Self::matching(file, "symbol".to_string(), tokenizer.token.to_string());
            *state = 3;
            return "Done".to_string();
        }
        match Grammar::class_var_dec_grammar(&tokenizer.token, &tokenizer.token_kind, state) {
            Ok((token, tag)) => {
                if *state == 1 {
                    writeln!(file, "<classVarDec>").unwrap();
                } else if *state == 0 {
                    writeln!(file, "</classVarDec>").unwrap();
                }
                *state += 1;
                Self::matching(file, tag, token)
            }
            Err(error) => {
                panic!("{error}");
            }
        }
    }

    pub fn subroutine_dec_compiler(
        file: &mut File,
        tokenizer: &mut JackTokenizer,
        state: &mut i8,
    ) -> String {
        match Grammar::subroutine_dec_grammar(&tokenizer.token, &tokenizer.token_kind, state) {
            Ok((token, tag)) => {
                if *state == 1 {
                    writeln!(file, "<subroutineDec>").unwrap();
                } else if *state == 0 {
                    writeln!(file, "</subroutineDec>").unwrap();
                }
                *state += 1;
                Self::matching(file, tag, token)
            }
            Err(error) => {
                panic!("{error}");
            }
        }
    }

    pub fn subroutine_call_compiler(
        file: &mut File,
        tokenizer: &mut JackTokenizer,
        state: &mut i8,
    ) -> String {
        if *state == 3 && tokenizer.token == "." && tokenizer.token_kind == TokenKind::Symbol {
            Self::matching(file, "symbol".to_string(), tokenizer.token.to_string());
            *state = 2;
            return "Done".to_string();
        }
        match Grammar::subroutine_call_grammar(&tokenizer.token, &tokenizer.token_kind, state) {
            Ok((token, tag)) => {
                if *state == 1 {
                    writeln!(file, "<subroutineCall>").unwrap();
                } else if *state == 0 {
                    writeln!(file, "</subroutineCall>").unwrap();
                }
                *state += 1;
                Self::matching(file, tag, token)
            }
            Err(error) => {
                panic!("{error}");
            }
        }
    }

    pub fn parameter_list_compiler(
        file: &mut File,
        tokenizer: &mut JackTokenizer,
        state: &mut i8,
    ) -> String {
        if tokenizer.token == ")" && *state == 1 {
            writeln!(file, "<parameterList>").unwrap();
            writeln!(file, "</parameterList>").unwrap();
            return String::from("SafePop");
        }
        if *state == 4 && tokenizer.token == "," && tokenizer.token_kind == TokenKind::Symbol  {
            Self::matching(file, "symbol".to_string(), tokenizer.token.to_string());
            *state = 2;
            return "Done".to_string();
        }  
        match Grammar::parameter_list_grammar(&tokenizer.token, &tokenizer.token_kind, state) {
            Ok((token, tag)) => {
                if *state == 1 {
                    writeln!(file, "<parameterList>").unwrap();
                } else if *state == 0 {
                    writeln!(file, "</parameterList>").unwrap();
                }
                *state += 1;
                Self::matching(file, tag, token)
            }
            Err(error) => {
                panic!("{error}");
            }
        }
    }

    #[rustfmt::skip]
    pub fn let_compiler(
        file: &mut File, 
        tokenizer: &mut JackTokenizer, 
        state: &mut i8) -> String {
        match Grammar::let_statement_grammar(&tokenizer.token, &tokenizer.token_kind, state) {
            Ok((token, tag)) => {
                if *state == 1 {
                    writeln!(file, "<letStatement>").unwrap();
                } else if *state == 0 {
                    writeln!(file, "</letStatement>").unwrap();
                }
                *state += 1;
                Self::matching(file, tag, token)
            }
            Err(error) => {
                panic!("{error}");
            }
        }
    }

    #[rustfmt::skip]
    pub fn term_compiler(
        file: &mut File, 
        tokenizer: &mut JackTokenizer,
        state: &mut i8
    ) -> String {
        match Grammar::term_grammar( &tokenizer.token.clone(), &tokenizer.get_context(1), &tokenizer.token_kind, state) {
            Ok((token, tag)) => {
                if *state == 1 {
                    writeln!(file, "<term>").unwrap();
                } else if *state == 0 {
                    writeln!(file, "</term>").unwrap();
                }
                *state += 1;
                if tokenizer.token == "~" {
                    *state = 3;
                }
                if tokenizer.token == "[" {
                    *state = 0;
                }
                if &tokenizer.get_context(1) == "." {
                    *state = i8::MIN;
                }
                Self::matching(file, tag, token)
            }
            Err(error) => {
                panic!("{error}");
            }
        }
    }

    pub fn while_compiler(
        file: &mut File,
        tokenizer: &mut JackTokenizer,
        state: &mut i8,
    ) -> String {
        match Grammar::while_statement_grammar(&tokenizer.token, &tokenizer.token_kind, state) {
            Ok((token, tag)) => {
                if *state == 1 {
                    writeln!(file, "<whileStatement>").unwrap();
                } else if *state == 0 {
                    writeln!(file, "</whileStatement>").unwrap();
                }
                *state += 1;
                Self::matching(file, tag, token)
            }
            Err(error) => {
                panic!("{error}");
            }
        }
    }

    pub fn parantheses_expressions_compiler(
        file: &mut File,
        tokenizer: &mut JackTokenizer,
        state: &mut i8,
    ) -> String {
        match Grammar::parantheses_expressions_grammar(&tokenizer.token, &tokenizer.token_kind, state) {
            Ok((token, tag)) => {
                *state += 1;
                Self::matching(file, tag, token)
            }
            Err(error) => {
                panic!("{error}");
            }
        }
    }

    pub fn brackets_expression_compiler(
        file: &mut File,
        tokenizer: &mut JackTokenizer,
        state: &mut i8,
    ) -> String {
        match Grammar::brackets_expressions_grammar(&tokenizer.token, &tokenizer.token_kind, state) {
            Ok((token, tag)) => {
                *state += 1;
                Self::matching(file, tag, token)
            }
            Err(error) => {
                panic!("{error}");
            }
        }
    }

    pub fn parantheses_expressions_list_compiler(
        file: &mut File,
        tokenizer: &mut JackTokenizer,
        state: &mut i8,
    ) -> String {
        match Grammar::parantheses_expressions_list_grammar(&tokenizer.token, &tokenizer.token_kind, state) {
            Ok((token, tag)) => {
                *state += 1;
                Self::matching(file, tag, token)
            }
            Err(error) => {
                panic!("{error}");
            }
        }
    }

    pub fn brackets_expression_list_compiler(
        file: &mut File,
        tokenizer: &mut JackTokenizer,
        state: &mut i8,
    ) -> String {
        match Grammar::brackets_expressions_list_grammar(&tokenizer.token, &tokenizer.token_kind, state) {
            Ok((token, tag)) => {
                *state += 1;
                Self::matching(file, tag, token)
            }
            Err(error) => {
                panic!("{error}");
            }
        }
    }

    pub fn if_compiler(file: &mut File, tokenizer: &mut JackTokenizer, state: &mut i8) -> String {
        match Grammar::if_statement_grammar(&tokenizer.token, &tokenizer.token_kind, state) {
            Ok((token, tag)) => {
                if *state == 1 {
                    writeln!(file, "<ifStatement>").unwrap();
                } else if *state == 0 {
                    writeln!(file, "</ifStatement>").unwrap();
                }
                *state += 1;
                Self::matching(file, tag, token)
            }
            Err(error) => {
                panic!("{error}");
            }
        }
    }

    pub fn statements_compiler(
        file: &mut File,
        tokenizer: &mut JackTokenizer,
        state: &mut i8,
    ) -> String {
        match Grammar::statements_grammar(&tokenizer.token, &tokenizer.token_kind, state) {
            Ok((token, tag)) => {
                if *state == 1 {
                    writeln!(file, "<statements>").unwrap();
                } else if *state == 0 {
                    writeln!(file, "</statements>").unwrap();
                }
                *state += 1;
                Self::matching(file, tag, token)
            }
            Err(error) => {
                panic!("{error}");
            }
        }
    }

    pub fn compile(&mut self) {
        if self.stack.is_empty() {
                self.stack.push(String::from("Class"));
        } 
        while self.tokenizer.has_more_token() {
            loop {
                let (func, state) = self.stack.get();
                let result = func(&mut self.file, &mut self.tokenizer, state);
                if result == "SafePop" {
                    self.stack.pop();
                    println!("{:?}", self.stack);
                    continue;
                } else if result == "SafeIterate" {
                    continue;
                } else if result == "Done" {
                    break;
                } else {
                    self.stack.push(result);
                    println!("{:?}", self.stack);
                    continue;
                }
            }
        }
    } 
}
