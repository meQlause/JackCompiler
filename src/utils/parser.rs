use core::panic;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use super::jack_tokenizer::JackTokenizer;
use std::vec;
use std::fmt;


pub struct StackCompiler {
    list_func :HashMap<String, Box<dyn FnMut(&mut File, &JackTokenizer, &mut i8) -> Option<String>>>,
    pointer: usize,
    stack_state: Vec<i8>,
    stack_compiler: Vec<String>
}
impl fmt::Debug for StackCompiler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StackCompiler")
            .field("stack_compiler", &self.stack_compiler)
            .finish()
    }
}
impl StackCompiler {
    pub fn new() -> StackCompiler {
        let mut list_func :HashMap<String, Box<dyn FnMut(&mut File, &JackTokenizer, &mut i8) -> Option<String>>> = HashMap::new();
        list_func.insert("expressionFn".to_string(), Box::new(CompilationEngine::expression_compiler));
        list_func.insert("expressionlistFn".to_string(), Box::new(CompilationEngine::expression_list_compiler));
        list_func.insert("whileFn".to_string(), Box::new(CompilationEngine::while_compiler));
        list_func.insert("statementsFn".to_string(), Box::new(CompilationEngine::statements_compiler));
        list_func.insert("termFn".to_string(), Box::new(CompilationEngine::term_compiler));
        list_func.insert("letFn".to_string(), Box::new(CompilationEngine::let_compiler));
        list_func.insert("ifFn".to_string(), Box::new(CompilationEngine::if_compiler));
        list_func.insert("classFn".to_string(), Box::new(CompilationEngine::class_compiler));
        list_func.insert("classvardecFn".to_string(), Box::new(CompilationEngine::class_var_dec_compiler));
        list_func.insert("subroutinedecFn".to_string(), Box::new(CompilationEngine::subroutine_dec_compiler));
        list_func.insert("parameterlistFn".to_string(), Box::new(CompilationEngine::parameter_list_compiler));
        list_func.insert("subroutinebodyFn".to_string(), Box::new(CompilationEngine::subroutine_body_compiler));
        list_func.insert("subroutinecallFn".to_string(), Box::new(CompilationEngine::subroutine_call_compiler));
        list_func.insert("doFn".to_string(), Box::new(CompilationEngine::do_compiler));
        list_func.insert("vardecFn".to_string(), Box::new(CompilationEngine::var_dec_compiler));
        list_func.insert("returnFn".to_string(), Box::new(CompilationEngine::return_compiler));
        Self { 
            list_func: list_func,
            pointer: 0, 
            stack_compiler: Vec::new(),
            stack_state: Vec::new(),
        }
    }
    pub fn push(&mut self, s:&String) {
        self.stack_compiler.push(s.to_string()); 
        self.stack_state.push(1);
        self.pointer = self.stack_compiler.len() - 1;
    }
    pub fn pop(&mut self) {
        self.stack_compiler.pop();
        self.stack_state.pop();
        if self.stack_compiler.len() != 0 {
            self.pointer = self.stack_compiler.len() - 1;
        }
    }
    pub fn get(&mut self) -> (&mut dyn FnMut(&mut File, &JackTokenizer, &mut i8) -> Option<String>, &mut i8) {
        let key = &self.stack_compiler[self.pointer];
        (self.list_func.get_mut(key).unwrap(),  &mut self.stack_state[self.pointer])
    }

    pub fn is_empty(&self) -> bool {
        self.stack_state.is_empty()
    }


}

#[derive(Debug)]

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

    fn class_compiler(file: &mut File, s:&JackTokenizer, state: &mut i8) -> Option<String> {
        if state == &mut 1i8 {
            if "class".to_string() != s.keyword.as_ref().unwrap().to_string() { panic!("not class statement"); }
            writeln!(file,"<class>").unwrap();
            writeln!(file,"{}", Self::parse(&"keyword".to_string(), &s.keyword.as_ref().unwrap().to_string())).unwrap();
        } else if state == &mut 2i8 {
            if let Some(class_name) = s.identifier.to_owned() {
            writeln!(file,"{}", Self::parse(&"identifier".to_string(), &class_name)).unwrap();
            } else {
                panic!("invalid class name");
            }
        } else if state == &mut 3i8 {
            if "{".to_string() != s.symbol.to_owned().unwrap().to_string() { panic!("expected {{, found {}", s.symbol.to_owned().unwrap().to_string()); }
            writeln!(file,"{}", Self::parse(&"symbol".to_string(), &s.symbol.to_owned().unwrap().to_string())).unwrap();
        } else if state == &mut 4i8 {
            if let Some(_) = s.keyword {
                *state += 1;
                return Some("classvardecFn".to_string());
            }
        } else if state == &mut 5i8 {
            if !vec!["constructor".to_string(), "function".to_string(), "method".to_string()].contains(&s.keyword.to_owned().unwrap()) { 
                *state = 4;
                return Some("SafeIterate".to_string()); 
            } else {
                *state += 1;
                return Some("subroutinedecFn".to_string());
            }
        } else if state == &mut 6i8 {
            if s.keyword.is_some() {
                if vec!["constructor".to_string(), "function".to_string(), "method".to_string()].contains(&s.keyword.to_owned().unwrap()) { 
                    *state = 5;
                    return Some("SafeIterate".to_string()); 
                } 
            }
            else {
                writeln!(file,"{}", Self::parse(&"symbol".to_string(), &s.symbol.to_owned().unwrap().to_string())).unwrap();
                writeln!(file,"</class>").unwrap();
            }
        }
        *state += 1;
        if *state == 7 { *state = 0; } 
        None
    }
    
    fn return_compiler(file: &mut File, s:&JackTokenizer, state: &mut i8) -> Option<String> {
        if state == &mut 1i8 {
            if "return".to_string() != s.keyword.as_ref().unwrap().to_string() { panic!("not class statement"); }
            writeln!(file,"<returnStatement>").unwrap();
            writeln!(file,"{}", Self::parse(&"keyword".to_string(), &s.keyword.as_ref().unwrap().to_string())).unwrap();
        } else if state == &mut 2i8 {
            if let Some(symbol) = &s.symbol { 
                if symbol == &";".to_string() {
                    writeln!(file,"{}", Self::parse(&"symbol".to_string(), &symbol.to_string())).unwrap();
                    writeln!(file,"</returnStatement>").unwrap();
                    *state = 0;
                    return None;
                }
            } else {
                *state += 1;
                return Some("expressionFn".to_string());
            }
        } else if state == &mut 3i8 {
            if let Some(symbol) = &s.symbol { 
                if symbol == &";".to_string() {
                    *state = 2;
                    return Some("SafeIterate".to_string());
                }
            }
            *state += 1;
            return Some("SafeIterate".to_string());
        } 
        *state += 1;
        None
    }

    fn class_var_dec_compiler(file: &mut File, s:&JackTokenizer, state: &mut i8) -> Option<String> {
        if state == &mut 1i8 {
            writeln!(file,"<classVarDec>");
            *state += 1;
            return Some("SafeIterate".to_string());
        } else if state == &mut 2i8 {
            if let Some(keyword) = s.keyword.to_owned() {
                if !vec!["static".to_string(), "field".to_string()].contains(&keyword) { 
                    writeln!(file,"</classVarDec>");
                    return Some("SafePop".to_string());
                }
                writeln!(file,"{}", Self::parse(&"keyword".to_string(), &s.keyword.as_ref().unwrap().to_string())).unwrap();   
            }
        } else if state == &mut 3i8 {
            if let Some(keyword) = s.keyword.to_owned() {
                if !vec!["int".to_string(), "char".to_string(), "boolean".to_string()].contains(&keyword) { panic!("need a subroutine declaration"); }
                writeln!(file,"{}", Self::parse(&"keyword".to_string(), &s.keyword.as_ref().unwrap().to_string())).unwrap();   
            } else if let Some(identifier) = s.identifier.to_owned() {
                writeln!(file,"{}", Self::parse(&"identifier".to_string(), &identifier)).unwrap();   
            } else { panic!("expected identifier"); }
        } else if state == &mut 4i8 {
            if let Some(identifier) = s.identifier.to_owned() {
                writeln!(file,"{}", Self::parse(&"identifier".to_string(), &identifier)).unwrap();
            }
        } else if state == &mut 5i8  { 
            if Some(",".to_string()) == s.symbol.to_owned() {
                writeln!(file,"{}", Self::parse(&"symbol".to_string(), &s.symbol.to_owned().unwrap().to_string())).unwrap();
                *state = 3;
            } else {
                *state += 1;
                return Some("SafeIterate".to_string());
            }
        } else if state == &mut 6i8  {
            if ";".to_string() == s.symbol.to_owned().unwrap().to_string() {
                writeln!(file,"{}", Self::parse(&"symbol".to_string(), &s.symbol.to_owned().unwrap().to_string())).unwrap();   
                writeln!(file,"</classVarDec>");
            } else { panic!("expected ; found after statement"); }
        }
        *state += 1;
        if state == &mut 7i8 { *state = 0; }
        None
    }

    fn subroutine_dec_compiler(file: &mut File, s:&JackTokenizer, state: &mut i8) -> Option<String> {
        if state == &mut 1i8 {
            writeln!(file,"<subroutineDec>");
            *state += 1;
            return Some("SafeIterate".to_string());
        } else if state == &mut 2i8 {
            if let Some(keyword) = s.keyword.to_owned() {
                if !vec!["constructor".to_string(), "function".to_string(), "method".to_string()].contains(&keyword) { panic!("need a subroutine declaration"); }
                writeln!(file,"{}", Self::parse(&"keyword".to_string(), &s.keyword.as_ref().unwrap().to_string())).unwrap();   
            } 
        } else if state == &mut 3i8 {
            if let Some(keyword) = s.keyword.to_owned() {
                if !vec!["void".to_string(), "int".to_string(), "char".to_string(), "boolean".to_string()].contains(&keyword) { panic!("need a subroutine identifier"); }
                writeln!(file,"{}", Self::parse(&"keyword".to_string(), &s.keyword.as_ref().unwrap().to_string())).unwrap();   
            } else if let Some(identifier) = s.identifier.to_owned() {
                writeln!(file,"{}", Self::parse(&"identifier".to_string(), &identifier)).unwrap();   
            }
        } else if state == &mut 4i8 {
            if let Some(identifier) = s.identifier.to_owned() {
                writeln!(file,"{}", Self::parse(&"identifier".to_string(), &identifier)).unwrap();   
            }
        } else if state == &mut 5i8  {
            if "(".to_string() == s.symbol.to_owned().unwrap().to_string() {
                writeln!(file,"{}", Self::parse(&"symbol".to_string(), &s.symbol.to_owned().unwrap().to_string())).unwrap();   
            } else { panic!("expected ( found after subroutine name"); }
        } else if state == &mut 6i8  {
            *state += 1;
            return Some("parameterlistFn".to_string());
        } else if state == &mut 7i8  {
            if ")".to_string() == s.symbol.to_owned().unwrap().to_string() {
                writeln!(file,"{}", Self::parse(&"symbol".to_string(), &s.symbol.to_owned().unwrap().to_string())).unwrap();   
            } else { panic!("expected )"); }
        } else if state == &mut 8i8  {
            *state += 1;
            return Some("subroutinebodyFn".to_string());
        } else if state == &mut 9i8  {
            writeln!(file,"</subroutineDec>");
            return Some("SafePop".to_string());
        }
        *state += 1;
        if state == &mut 10i8 { *state = 0; }
        None
    }

    fn subroutine_call_compiler(file: &mut File, s:&JackTokenizer, state: &mut i8) -> Option<String> {
        if state == &mut 1i8 {
            writeln!(file,"<subroutineCall>");
            writeln!(file,"{}", Self::parse(&"identifier".to_string(), &s.identifier.as_ref().unwrap().to_string())).unwrap();   
        } else if state == &mut 2i8 {
            if let Some(symbol) = s.symbol.to_owned() {
                if "(".to_string() == symbol { 
                    writeln!(file,"{}", Self::parse(&"symbol".to_string(), &symbol.to_string())).unwrap();   
                    *state = 4;
                } else if ".".to_string() == symbol {
                    writeln!(file,"{}", Self::parse(&"symbol".to_string(), &symbol.to_string())).unwrap();   
                } else {panic!("expected symbol")}
            } 
        } else if state == &mut 3i8 {
            writeln!(file,"{}", Self::parse(&"identifier".to_string(), &s.identifier.to_owned().unwrap())).unwrap();   
        } else if state == &mut 4i8 {
            if let Some(symbol) = s.symbol.to_owned() {
                if "(".to_string() == symbol { 
                    writeln!(file,"{}", Self::parse(&"symbol".to_string(), &symbol.to_string())).unwrap();
                }
            }
        } else if state == &mut 5i8 {
            *state += 1;
            return Some("expressionlistFn".to_string());
        } else if state == &mut 6i8 {
            if let Some(symbol) = s.symbol.to_owned() {
                if ")".to_string() == symbol { 
                    writeln!(file,"{}", Self::parse(&"symbol".to_string(), &symbol.to_string())).unwrap();
                    writeln!(file,"</subroutineCall>");
                } else { panic!("expected symbol"); }
            }
        }
        *state += 1;
        if state == &mut 7i8 { *state = 0; }
        None

    }
    fn parameter_list_compiler(file: &mut File, s:&JackTokenizer, state: &mut i8) -> Option<String> {
        if state == &mut 1i8 {
            writeln!(file,"<parameterList>");
            *state += 1;
            return Some("SafeIterate".to_string());
        } else if state == &mut 2i8 {
            if let Some(keyword) = s.keyword.to_owned() {
                if !vec!["int".to_string(), "char".to_string(), "boolean".to_string()].contains(&keyword) { panic!("need a subroutine declaration"); }
                writeln!(file,"{}", Self::parse(&"keyword".to_string(), &s.keyword.as_ref().unwrap().to_string())).unwrap();   
            } else if let Some(identifier) = s.identifier.to_owned() {
                writeln!(file,"{}", Self::parse(&"identifier".to_string(), &identifier)).unwrap();   
            } else {
                writeln!(file,"</parameterList>");
                return Some("SafePop".to_string())
            }
        } else if state == &mut 3i8 {
            if let Some(identifier) = s.identifier.to_owned() {
                writeln!(file,"{}", Self::parse(&"identifier".to_string(), &identifier)).unwrap();   
            } else { panic!("expect variable name"); }
        } else if state == &mut 4i8 {
            if let Some(symbol) = s.symbol.to_owned() { 
                if symbol == ",".to_string() {
                    writeln!(file,"{}", Self::parse(&"symbol".to_string(), &symbol.to_string())).unwrap();   
                    *state = 2;
                    return None;
                }
                writeln!(file,"</parameterList>");
                return Some("SafePop".to_string())
            }
        } 
        *state += 1;
        if state == &mut 5i8 { *state = 0; }
        None
    }

    fn subroutine_body_compiler(file: &mut File, s:&JackTokenizer, state: &mut i8) -> Option<String> {
        if state == &mut 1i8 {
            writeln!(file,"<subroutineBody>").unwrap();
            if "{".to_string() != s.symbol.to_owned().unwrap().to_string() { panic!("expected {{"); }
            writeln!(file,"{}", Self::parse(&"symbol".to_string(), &s.symbol.as_ref().unwrap().to_string())).unwrap();
        } else if state == &mut 2i8 {
            if let Some(keyword) = &s.keyword  {
                if keyword == &"var" {
                    return Some("vardecFn".to_string()); 
                }
            }
            *state += 1;
            return Some("SafeIterate".to_string());
        } else if state == &mut 3i8 {
            *state += 1;
            return Some("statementsFn".to_string()); 
        } else if state == &mut 4i8 {
            if "}".to_string() != s.symbol.to_owned().unwrap().to_string() { panic!("expected {{"); }
                writeln!(file,"{}", Self::parse(&"symbol".to_string(), &s.symbol.as_ref().unwrap().to_string())).unwrap();
                writeln!(file,"</subroutineBody>").unwrap();
            }
        *state += 1;
        if *state == 5 { *state = 0; } 
        None
    }

    fn var_dec_compiler(file: &mut File, s:&JackTokenizer, state: &mut i8) -> Option<String> {
        if state == &mut 1i8 {
            if s.keyword.to_owned().unwrap() != "var".to_string() {panic!("expeced var, for var declaration")}
            writeln!(file,"<varDec>");
            writeln!(file,"{}", Self::parse(&"keyword".to_string(), &s.keyword.as_ref().unwrap().to_string())).unwrap();   
        } else if state == &mut 2i8 {
            if let Some(keyword) = s.keyword.to_owned() {
                if !vec!["int".to_string(), "char".to_string(), "boolean".to_string()].contains(&keyword) { panic!("need a subroutine declaration"); }
                writeln!(file,"{}", Self::parse(&"keyword".to_string(), &s.keyword.as_ref().unwrap().to_string())).unwrap();   
            } else if let Some(identifier) = s.identifier.to_owned() {
                writeln!(file,"{}", Self::parse(&"identifier".to_string(), &identifier)).unwrap();   
            } else { panic!("expected identifier"); }
        } else if state == &mut 3i8 {
            if let Some(identifier) = s.identifier.to_owned() {
                writeln!(file,"{}", Self::parse(&"identifier".to_string(), &identifier)).unwrap();
            }
        } else if state == &mut 4i8  { 
            if Some(','.to_string()) == s.symbol.to_owned() {
                writeln!(file,"{}", Self::parse(&"symbol".to_string(), &s.symbol.to_owned().unwrap().to_string())).unwrap();
                *state = 2;
            } else {
                *state += 1;
                return Some("SafeIterate".to_string());
            }
        } else if state == &mut 5i8  {
            println!("{:?}", s.symbol.to_owned());
            if ";".to_string() == s.symbol.to_owned().unwrap().to_string() {
                writeln!(file,"{}", Self::parse(&"symbol".to_string(), &s.symbol.to_owned().unwrap().to_string())).unwrap();   
                writeln!(file,"</varDec>");
            } else { panic!("expected ; found after statement"); }
        }
        *state += 1;
        if state == &mut 6i8 { *state = 0; }
        None
    }

    fn expression_compiler(file: &mut File, s:&JackTokenizer, state: &mut i8) -> Option<String> {
        if state == &mut 1i8 {
            writeln!(file,"<expression>").unwrap();
            *state += 1;
            return Some("termFn".to_string());
        } else if state == &mut 2i8 { 
            if s.symbol.is_some() {
                if !vec!["+".to_string(), "*".to_string(), "/".to_string(), "-".to_string(), "=".to_string(), "<".to_string(), ">".to_string(), "&".to_string(), "||".to_string()].contains(&s.symbol.to_owned().unwrap().to_string()) {
                    writeln!(file,"</expression>").unwrap();
                    return Some("SafePop".to_string());
                } else {
                    writeln!(file,"{}", Self::parse(&"symbol".to_string(), &s.symbol.to_owned().unwrap().to_string())).unwrap();
                }
            } else {
                *state = 3;
            }
        } else if state == &mut 3i8 {
            *state = 2;
            return Some("termFn".to_string());
        } 
        *state += 1;
        if *state == 5 { *state = 0; }
        None
    }

    fn expression_list_compiler(file: &mut File, s:&JackTokenizer, state: &mut i8) -> Option<String> {
        if state == &mut 1i8 {
            if s.symbol == Some(")".to_string()) {
                writeln!(file,"<expressionList>");
                writeln!(file,"</expressionList>");
                return Some("SafePop".to_string());
            }
            writeln!(file,"<expressionList>");
            *state += 1;
            return Some("SafeIterate".to_string());
        } else if state == &mut 2i8 {
            *state += 1;
            return Some("expressionFn".to_string());
        } else if state == &mut 3i8 {
            if let Some(symbol) = s.symbol.to_owned() { 
                if symbol == ",".to_string() {
                    writeln!(file,"{}", Self::parse(&"symbol".to_string(), &symbol.to_string())).unwrap();   
                    *state = 2;
                    return None;
                } else {
                    writeln!(file,"</expressionList>");
                    return Some("SafePop".to_string());
                }
            }
        }
        *state += 1;
        if state == &mut 4i8 { *state = 0; }
        None
    }
    
    fn let_compiler(file: &mut File, s:&JackTokenizer, state: &mut i8) -> Option<String> {
        if state == &mut 1i8 {
            writeln!(file,"<letStatement>").unwrap();
            if let Some(keyword) = &s.keyword {
                writeln!(file, "{}", Self::parse(&"keyword".to_string(), keyword));
            }
        } else if state == &mut 2i8 {
            if let Some(syntax) = s.identifier.to_owned() {
                writeln!(file,"{}", Self::parse(&"identifier".to_string(), &syntax.to_string())).unwrap();              
            } else { panic!("need a variable name") }
        } else if state == &mut 3i8 {
            if let Some(symbol) = s.symbol.to_owned() {
                if "[".to_string() == symbol {
                    writeln!(file,"{}", Self::parse(&"symbol".to_string(), &symbol.to_string())).unwrap();              
                } else if "=".to_string() == symbol {
                    writeln!(file,"{}", Self::parse(&"symbol".to_string(), &symbol.to_string())).unwrap();
                    *state = 6;              
                }
            }
        } else if state == &mut 4i8 {
            *state += 1;
            return Some("expressionFn".to_string());
        } else if state == &mut 5i8 {
            if let Some(symbol) = s.symbol.to_owned() {
                if "]".to_string() == symbol {
                    writeln!(file,"{}", Self::parse(&"symbol".to_string(), &symbol.to_string())).unwrap();
                }
            }
        } else if state == &mut 6i8 {
            if let Some(symbol) = s.symbol.to_owned() {
                writeln!(file,"{}", Self::parse(&"symbol".to_string(), &symbol.to_string())).unwrap();         
            } else { panic!("expected ="); }
        } else if state == &mut 7i8 {
            *state += 1;
            return Some("expressionFn".to_string());
        } else if state == &mut 8i8 {
            writeln!(file,"{}", Self::parse(&"symbol".to_string(), &s.symbol.to_owned().unwrap().to_string())).unwrap();              
            writeln!(file,"</letStatement>").unwrap();
        }
        *state += 1;
        if *state == 9 { *state = 0; }
        None
    }

    fn term_compiler(file: &mut File, s:&JackTokenizer, state: &mut i8) -> Option<String> {
        if state == &mut 1i8 {
            writeln!(file,"<term>");
            if let Some(symbol) = s.symbol.to_owned() { 
                if symbol == "(".to_string() {
                    writeln!(file,"{}", Self::parse(&"symbol".to_string(), &symbol.to_string())).unwrap();
                    *state = 2;
                } 
                if symbol == "~".to_string() || symbol == "-".to_string() { 
                    writeln!(file,"{}", Self::parse(&"symbol".to_string(), &symbol.to_string())).unwrap();
                    *state = 6;
                }
            } else if let Some(syntax) = &s.int_val {
                writeln!(file,"{}", Self::parse(&"integerConstant".to_string(), &syntax.to_string())).unwrap();
                *state = 7;
            } else if let Some(syntax) = s.identifier.to_owned() {
                writeln!(file,"{}", Self::parse(&"identifier".to_string(), &syntax.to_string())).unwrap();    
            } else if let Some(syntax) = s.string_val.to_owned() {
                writeln!(file,"{}", Self::parse(&"stringConstant".to_string(), &syntax.to_string())).unwrap();
                *state = 7;
            }  else if let Some(syntax) = s.keyword.to_owned() {
                    if vec!["true".to_string(), "false".to_string(), "null".to_string(), "this".to_string()].contains(&syntax) {
                        writeln!(file,"{}", Self::parse(&"keywordConstant".to_string(), &syntax.to_string())).unwrap();
                        *state = 7;
                    } 
            }  
        }
        else if state == &mut 2i8 {
            if ";".to_string() == s.symbol.to_owned().unwrap().to_string() { 
                writeln!(file,"</term>");
                *state = 0;
                return Some("SafePop".to_string());
            } else if "[".to_string() == s.symbol.to_owned().unwrap().to_string() {
                *state = 4;
            } else {
                writeln!(file,"</term>");
                *state = 0;
                return Some("SafePop".to_string());
            }
        }

        else if state == &mut 3i8 {
            *state += 1;
            return Some("expressionFn".to_string());
        } else if state == &mut 4i8 {
            if ")".to_string() != s.symbol.to_owned().unwrap().to_string() { panic!("expected ), found {}", s.symbol.to_owned().unwrap().to_string()); }
            writeln!(file,"{}", Self::parse(&"symbol".to_string(), &s.symbol.to_owned().unwrap().to_string())).unwrap();
            writeln!(file,"</term>");
            *state = 0;
            return None;
        } 
        
        else if state == &mut 5i8 {
            *state += 1;
            return Some("expressionFn".to_string());
        } else if state == &mut 6i8 {
            if "]".to_string() != s.symbol.to_owned().unwrap().to_string() { panic!("expected ), found {}", s.symbol.to_owned().unwrap().to_string()); }
            writeln!(file,"{}", Self::parse(&"symbol".to_string(), &s.symbol.to_owned().unwrap().to_string())).unwrap();
            writeln!(file,"</term>");
            *state = 0;
            return None;
        } 
        else if state == &mut 7i8 {
            *state += 1;
            return Some("termFn".to_string());
        }
        else if state == &mut 8i8 {
            writeln!(file,"</term>");
            return Some("SafePop".to_string());
        }
        *state += 1;
        if state ==  &mut 10i8 { *state = 0; }
        None
    }

    fn statements_compiler(file: &mut File, s:&JackTokenizer, state: &mut i8) -> Option<String> {
        let statements = vec!["let".to_string(), "while".to_string(), "if".to_string(), "do".to_string(), "return".to_string()];
        if state == &mut 1i8 {
            writeln!(file,"<statements>");
            *state += 1;
            return Some("SafeIterate".to_string());
        } else if state == &mut 2i8 {
            if let Some(syntax) = &s.keyword {
                println!("{}", syntax);
                if !statements.contains(syntax) {panic!("{} not a statement", syntax)}
                if syntax == &statements[0] {return Some("letFn".to_string());}
                if syntax == &statements[1] {return Some("whileFn".to_string());}
                if syntax == &statements[2] {return Some("ifFn".to_string());}
                if syntax == &statements[3] {return Some("doFn".to_string());}
                if syntax == &statements[4] {return Some("returnFn".to_string());}
            } 
            *state += 1;
            return Some("SafeIterate".to_string())
        } else if state == &mut 3i8 {
            writeln!(file,"</statements>");
            return Some("SafePop".to_string())
        }
        *state += 1;
        if state == &mut 4i8 {*state = 0;}
        None
    }
    fn do_compiler(file: &mut File, s:&JackTokenizer, state: &mut i8) -> Option<String> {
        if state == &mut 1i8 {
            if "do".to_string() != s.keyword.as_ref().unwrap().to_string() { panic!("not do statement"); }
            writeln!(file,"<doStatement>").unwrap();
            writeln!(file,"{}", Self::parse(&"keyword".to_string(), &s.keyword.as_ref().unwrap().to_string())).unwrap();
        } else if state == &mut 2i8 {
            *state += 1;
            return Some("subroutinecallFn".to_string());
        }
        else if state == &mut 3i8 {
            writeln!(file,"{}", Self::parse(&"symbol".to_string(), &s.symbol.as_ref().unwrap().to_string())).unwrap();
            writeln!(file,"</doStatement>").unwrap();
        }
        *state += 1;
        if state == &mut 4i8 {*state = 0;}
        None
    }
    fn while_compiler(file: &mut File, syntax: &JackTokenizer, state: &mut i8) -> Option<String> {
        if state == &mut 1i8 {
            if "while".to_string() != syntax.keyword.as_ref().unwrap().to_string() { panic!("not while statement"); }
            writeln!(file,"<whileStatement>").unwrap();
            writeln!(file,"{}", Self::parse(&"keyword".to_string(), &syntax.keyword.as_ref().unwrap().to_string())).unwrap();
        } else if state == &mut 2i8 {
            if "(".to_string() != syntax.symbol.to_owned().unwrap().to_string() { panic!("expected (, found {}", syntax.symbol.to_owned().unwrap().to_string()); }
            writeln!(file,"{}", Self::parse(&"symbol".to_string(), &syntax.symbol.to_owned().unwrap().to_string())).unwrap();
        } else if state == &mut 3i8 {
            *state += 1;
            return Some("expressionFn".to_string());
        } else if state == &mut 4i8 {
            if ")".to_string() != syntax.symbol.to_owned().unwrap().to_string() { panic!("expected ), found {}", syntax.symbol.to_owned().unwrap().to_string()); }
            writeln!(file,"{}", Self::parse(&"symbol".to_string(), &syntax.symbol.to_owned().unwrap().to_string())).unwrap();
        } else if state == &mut 5i8 {
            if "{".to_string() != syntax.symbol.to_owned().unwrap().to_string() { panic!("expected {{, found {}", syntax.symbol.to_owned().unwrap().to_string()); }
            writeln!(file,"{}", Self::parse(&"symbol".to_string(), &syntax.symbol.to_owned().unwrap().to_string())).unwrap();
        } else if state == &mut 6i8 {
            *state += 1;
            return Some("statementsFn".to_string());
        } else if state == &mut 7i8 {
            writeln!(file,"{}", Self::parse(&"symbol".to_string(), &syntax.symbol.to_owned().unwrap().to_string())).unwrap();
            writeln!(file,"</whileStatement>").unwrap();
        }
        *state += 1;
        if *state == 8 { *state = 0; } 
        None

    }

    fn if_compiler(file: &mut File, syntax: &JackTokenizer, state: &mut i8) -> Option<String> {
        if state == &mut 1i8 {
            if "if".to_string() != syntax.keyword.as_ref().unwrap().to_string() { panic!("not while statement"); }
            writeln!(file,"<ifStatement>").unwrap();
            writeln!(file,"{}", Self::parse(&"keyword".to_string(), &syntax.keyword.as_ref().unwrap().to_string())).unwrap();
        } else if state == &mut 2i8 {
            if "(".to_string() != syntax.symbol.to_owned().unwrap().to_string() { panic!("expected (, found {}", syntax.symbol.to_owned().unwrap().to_string()); }
            writeln!(file,"{}", Self::parse(&"symbol".to_string(), &syntax.symbol.to_owned().unwrap().to_string())).unwrap();
        } else if state == &mut 3i8 {
            *state += 1;
            return Some("expressionFn".to_string());
        } else if state == &mut 4i8 {
            if ")".to_string() != syntax.symbol.to_owned().unwrap().to_string() { panic!("expected ), found {}", syntax.symbol.to_owned().unwrap().to_string()); }
            writeln!(file,"{}", Self::parse(&"symbol".to_string(), &syntax.symbol.to_owned().unwrap().to_string())).unwrap();
        } else if state == &mut 5i8 {
            if "{".to_string() != syntax.symbol.to_owned().unwrap().to_string() { panic!("expected {{, found {}", syntax.symbol.to_owned().unwrap().to_string()); }
            writeln!(file,"{}", Self::parse(&"symbol".to_string(), &syntax.symbol.to_owned().unwrap().to_string())).unwrap();
        } else if state == &mut 6i8 {
            *state += 1;
            return Some("statementsFn".to_string());
        } else if state == &mut 7i8 {
            println!("{}", &syntax.symbol.to_owned().unwrap().to_string());
            writeln!(file,"{}", Self::parse(&"symbol".to_string(), &syntax.symbol.to_owned().unwrap().to_string())).unwrap();
        } else if state == &mut 8i8 {
            if let Some(keyword) = &syntax.keyword {
                if keyword == &"else".to_string() {
                    writeln!(file,"{}", Self::parse(&"keyword".to_string(), &keyword.to_string())).unwrap();
                    *state = 5;
                    return None;
                }
            }
            *state += 1;
            return Some("SafeIterate".to_string());
        
        } else if state == &mut 9i8 {
            writeln!(file,"</ifStatement>").unwrap();
            return Some("SafePop".to_string());
        }

        *state += 1;
        if *state == 9 { *state = 0; } 
        None

    }

    fn parse(p:&String, c:&String) -> String {
        format!("<{p}>{c}</{p}>", p=p, c=c)
    }
    pub fn compile(&mut self, s:&mut JackTokenizer) {
        if self.stack.is_empty() {self.stack.push(&"classFn".to_string());}
        loop {
            let (func, state) = self.stack.get();
            match func(&mut self.file, s, state) {  
                Some(f) => {
                    if f == "SafePop".to_string() { 
                        println!("pop {:?}", self.stack);
                        self.stack.pop();
                        continue;
                    }
                    if f == "SafeIterate".to_string() { 
                        continue;
                    }
                    if (f == "termFn".to_string() && s.identifier.is_some()) && (s.get_context(1).unwrap() == ".".to_string() || s.get_context(1).unwrap() == "(".to_string()) { 
                        println!("{:?} ini", s.get_context(1));
                        println!("{:?} ini", s.get_context(1));
                        self.stack.push(&"subroutinecallFn".to_string());
                        continue;
                    }
                    self.stack.push(&f);
                    println!("push {:?}", self.stack);
                    continue;
                },
                None => {
                    if state == &mut 0i8 { self.stack.pop() }
                    println!("pop {:?}", self.stack);
                    break
                },
            };
        }
    }
}
