use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::vec;


pub enum Grammar {
    WhileStatement(
        String, 
        char, 
        String,
        char, 
        char, 
        String,
        char,
        i8
    ),
    Expression(
        String,
        Vec<char>,
        String,
    )
}
pub struct StackCompiler {
    list_func :HashMap<String, Box<dyn FnMut(&mut File, String, &mut Grammar) -> Option<String>>>,
    stack: Vec<String>,
    valid_stack: Vec<String>
}
impl StackCompiler {
    pub fn new() -> StackCompiler {
        let mut list_func :HashMap<String, Box<dyn FnMut(&mut File, String, &mut Grammar) -> Option<String>>> = HashMap::new();
        list_func.insert("expressionFn".to_string(), Box::new(CompilationEngine::expression_compiler));
        list_func.insert("statementsFn".to_string(), Box::new(CompilationEngine::statements_compiler));
        list_func.insert("varnameFn".to_string(), Box::new(CompilationEngine::varname_compiler));
        Self { 
            list_func: list_func, 
            stack: Vec::new(),
            valid_stack: vec!["expressionFn".to_string(), "statementsFn".to_string(), "varnameFn".to_string()]
        }
    }
    pub fn push(&mut self, s:String) {
        if self.valid_stack.contains(&s) { self.stack.push(s); }
    }
    pub fn get(&self) -> &Box<dyn FnMut(&mut File, String, &mut Grammar) -> Option<String>> {
        let key = &self.stack[self.stack.len()-1];
        self.list_func.get(key).unwrap()
    }
}
pub struct CompilationEngine {
    file: File,
    while_statement: Grammar,
    stack : StackCompiler,
}

impl CompilationEngine {
    pub fn new(file_input:File) -> CompilationEngine {
        CompilationEngine { 
            file: file_input, 
            stack: StackCompiler::new(),
            while_statement: Grammar::WhileStatement(
                "while".to_string(), 
                '(', 
                "expressionFn".to_string(), 
                ')', 
                '{', 
                "statementFn".to_string(),  
                '}',
                0
            )
        }
    }
    
    fn varname_compiler(file: &mut File, s:String, grammar: &mut Grammar) -> Option<String> {
        writeln!(file,"<expression>");
        writeln!(file,"</expression>");
        None
    }

    fn expression_compiler(file: &mut File, s:String, grammar: &mut Grammar) -> Option<String> {
        writeln!(file,"<expression>");
        writeln!(file,"</expression>");
        None
    }

    fn term_compiler(file: &mut File, s:String, grammar: &mut Grammar) -> Option<String> {
        writeln!(file,"<term>");
        writeln!(file,"</term>");
        None
    }

    fn statements_compiler(file: &mut File, s:String, grammar: &mut Grammar) -> Option<String> {
        writeln!(file,"<statements>");
        writeln!(file,"</statements>");
        None
    }

    fn while_compiler(file: &mut File, syntax: String, grammar: &mut Grammar) -> Option<String> {
        match grammar {
            Grammar::WhileStatement(
                a, 
                b, 
                c, 
                d, 
                e, 
                f, 
                g, 
                state) => {
                    if *state == 0 {
                        if *a != syntax { panic!("not while statement"); }
                        writeln!(file,"<whileStatements>").unwrap();
                        writeln!(file,"{}", Self::parse("keyword".to_string(), syntax)).unwrap();
                    } else if *state == 1 {
                        if *b != syntax.chars().next().unwrap() { panic!("expected {}, found {}", *b, syntax); }
                        writeln!(file,"{}", Self::parse("symbol".to_string(), syntax)).unwrap();
                    } else if *state == 2 {
                        return Some(c.to_string());
                    } else if *state == 3 {
                        if *d != syntax.chars().next().unwrap() { panic!("expected {}, found {}", *b, syntax); }
                        writeln!(file,"{}", Self::parse("symbol".to_string(), syntax)).unwrap();
                    } else if *state == 4 {
                        if *e != syntax.chars().next().unwrap() { panic!("expected {}, found {}", *b, syntax); }
                        writeln!(file,"{}", Self::parse("symbol".to_string(), syntax)).unwrap();
                    } else if *state == 4 {
                        return Some(f.to_string());
                    } else if *state == 4 {
                        if *g != syntax.chars().next().unwrap() { panic!("expected {}, found {}", *b, syntax); }
                        writeln!(file,"{}", Self::parse("symbol".to_string(), syntax)).unwrap();
                        writeln!(file,"</whileStatements>").unwrap();
                    }
                    *state += 1;
                    if *state == 5i8 { *state = 0; } 
                    None
                }
                _ => panic!("not while statements")
        }
    }

    fn parse(p:String, c:String) -> String {
        format!("<{p}>{c}</{p}>", p=p, c=c)
    }
    pub fn compile(&mut self, s:String) {
        todo!()
    }
}
