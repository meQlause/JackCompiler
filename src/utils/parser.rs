use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use super::jack_tokenizer::JackTokenizer;
use std::vec;

pub struct StackCompiler {
    list_func :HashMap<String, Box<dyn FnMut(&mut File, &JackTokenizer, &mut i8) -> Option<String>>>,
    valid_stack: Vec<String>,
    pointer: usize,
    stack_state: Vec<i8>,
    stack_compiler: Vec<String>
}
impl StackCompiler {
    pub fn new() -> StackCompiler {
        let mut list_func :HashMap<String, Box<dyn FnMut(&mut File, &JackTokenizer, &mut i8) -> Option<String>>> = HashMap::new();
        list_func.insert("expressionFn".to_string(), Box::new(CompilationEngine::expression_compiler));
        list_func.insert("whileFn".to_string(), Box::new(CompilationEngine::while_compiler));
        list_func.insert("statementsFn".to_string(), Box::new(CompilationEngine::statements_compiler));
        list_func.insert("varnameFn".to_string(), Box::new(CompilationEngine::varname_compiler));
        list_func.insert("termFn".to_string(), Box::new(CompilationEngine::term_compiler));
        Self { 
            list_func: list_func,
            pointer: 0, 
            stack_compiler: Vec::new(),
            stack_state: Vec::new(),
            valid_stack: vec!["expressionFn".to_string(), "whileFn".to_string() ,"statementsFn".to_string(), "varnameFn".to_string(), "termFn".to_string()]
        }
    }
    pub fn push(&mut self, s:&String) {
        self.stack_compiler.push(s.to_string()); 
        self.stack_state.push(0);
        self.pointer = self.stack_compiler.len() - 1;
    }
    pub fn pop(&mut self) {
        self.stack_compiler.pop();
        self.stack_state.pop();
        self.pointer = self.stack_compiler.len() - 1;
    }
    pub fn get(&mut self) -> (&mut dyn FnMut(&mut File, &JackTokenizer, &mut i8) -> Option<String>, &mut i8) {
        let key = &self.stack_compiler[self.pointer];
        (self.list_func.get_mut(key).unwrap(),  &mut self.stack_state[self.pointer])
    }

    pub fn is_empty(&self) -> bool {
        self.stack_state.is_empty()
    }
}


pub struct CompilationEngine {
    file: File,
    stack : StackCompiler,
}

impl CompilationEngine {
    pub fn new(file_input:File) -> CompilationEngine {
        CompilationEngine { 
            file: file_input, 
            stack: StackCompiler::new(),
        }
    }
    
    fn varname_compiler(file: &mut File, s:&JackTokenizer, state: &mut i8) -> Option<String> {
        writeln!(file,"<expression>");
        writeln!(file,"</expression>");
        None
    }

    fn constant_compiler(file: &mut File, s:&JackTokenizer, state: &mut i8) -> Option<String> {
        writeln!(file,"<expression>");
        writeln!(file,"</expression>");
        None
    }

    fn expression_compiler(file: &mut File, syntax:&JackTokenizer, state: &mut i8) -> Option<String> {
        if !vec!["+".to_string(), "-".to_string(), "=".to_string(), "<".to_string(), ">".to_string()].contains(&syntax.symbol.unwrap().to_string()) && state == &mut 1i8 {
            if syntax.symbol.unwrap().to_string() != ")".to_string() { panic!("add ')' after expression")} 
            writeln!(file,"{}", Self::parse(&"symbol".to_string(), &syntax.symbol.unwrap().to_string())).unwrap();
            writeln!(file,"</expression>").unwrap();
            *state = 0;
            return None; 
        }
        if state == &mut 0i8 {
            writeln!(file,"<expression>").unwrap();
            *state += 1;
            return Some("termFn".to_string());
        } else if state == &mut 1i8 {
            writeln!(file,"{}", Self::parse(&"symbol".to_string(), &syntax.symbol.unwrap().to_string())).unwrap();
        } else if state == &mut 2i8 {
            *state += 1;
            return Some("termFn".to_string());
        } else if state == &mut 3i8 {
            writeln!(file,"</expression>").unwrap();
        }
        *state += 1;
        if *state == 4 { *state = 0; }
        None
    }
    


    fn term_compiler(file: &mut File, s:&JackTokenizer, state: &mut i8) -> Option<String> {
        writeln!(file,"<term>");
        writeln!(file,"</term>");
        None
    }

    fn statements_compiler(file: &mut File, s:&JackTokenizer, state: &mut i8) -> Option<String> {
        writeln!(file,"<statements>");
        writeln!(file,"</statements>");
        None
    }

    fn while_compiler(file: &mut File, syntax: &JackTokenizer, state: &mut i8) -> Option<String> {
        if state == &mut 0i8 {
            if "while".to_string() != syntax.keyword.as_ref().unwrap().to_string() { panic!("not while statement"); }
            writeln!(file,"<whileStatements>").unwrap();
            writeln!(file,"{}", Self::parse(&"keyword".to_string(), &syntax.keyword.as_ref().unwrap().to_string())).unwrap();
        } else if state == &mut 1i8 {
            if "(".to_string() != syntax.symbol.unwrap().to_string() { panic!("expected (, found {}", syntax.symbol.unwrap().to_string()); }
            writeln!(file,"{}", Self::parse(&"symbol".to_string(), &syntax.symbol.unwrap().to_string())).unwrap();
        } else if state == &mut 2i8 {
            *state += 1;
            return Some("expressionFn".to_string());
        } else if state == &mut 3i8 {
            if ")".to_string() != syntax.symbol.unwrap().to_string() { panic!("expected ), found {}", syntax.symbol.unwrap().to_string()); }
            writeln!(file,"{}", Self::parse(&"symbol".to_string(), &syntax.symbol.unwrap().to_string())).unwrap();
        } else if state == &mut 4i8 {
            if "{".to_string() != syntax.symbol.unwrap().to_string() { panic!("expected {{, found {}", syntax.symbol.unwrap().to_string()); }
            writeln!(file,"{}", Self::parse(&"symbol".to_string(), &syntax.symbol.unwrap().to_string())).unwrap();
        } else if state == &mut 5i8 {
            *state += 1;
            return Some("statementsFn".to_string());
        } else if state == &mut 6i8 {
            writeln!(file,"{}", Self::parse(&"symbol".to_string(), &syntax.symbol.unwrap().to_string())).unwrap();
            writeln!(file,"</whileStatements>").unwrap();
        }
        *state += 1;
        if *state == 7 { *state = 0; } 
        None

    }

    fn parse(p:&String, c:&String) -> String {
        format!("<{p}>{c}</{p}>", p=p, c=c)
    }
    pub fn compile(&mut self, s:&JackTokenizer) {
        if self.stack.is_empty() {self.stack.push(&"whileFn".to_string());}
        loop {
            let (func, state) = self.stack.get();
            match func(&mut self.file, s, state) {
                Some(f) => {
                    self.stack.push(&f);
                    continue;
                },
                None => break,
            };
        }
    }
}
