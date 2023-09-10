use crate::prelude::*;
use crate::prelude::panic;
type CompilerFunc = dyn FnMut(&mut File, &mut JackTokenizer, &mut i8) -> Option<String>;

pub struct StackCompiler {
    list_func: HashMap<String, Box<CompilerFunc>>,
    pointer: usize,
    stack_state: Vec<i8>,
    stack_compiler: Vec<String>,
}
impl Debug for StackCompiler {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("StackCompiler")
            .field("stack_compiler", &self.stack_compiler)
            .finish()
    }
}
impl Default for StackCompiler {
    fn default() -> StackCompiler {
        let mut list_func: HashMap<String, Box<CompilerFunc>> = HashMap::new();
        list_func.insert(
            "expressionFn".to_string(),
            Box::new(CompilationEngine::expression_compiler),
        );
        list_func.insert(
            "expressionlistFn".to_string(),
            Box::new(CompilationEngine::expression_list_compiler),
        );
        list_func.insert(
            "whileFn".to_string(),
            Box::new(CompilationEngine::while_compiler),
        );
        list_func.insert(
            "statementsFn".to_string(),
            Box::new(CompilationEngine::statements_compiler),
        );
        list_func.insert(
            "termFn".to_string(),
            Box::new(CompilationEngine::term_compiler),
        );
        list_func.insert(
            "letFn".to_string(),
            Box::new(CompilationEngine::let_compiler),
        );
        list_func.insert(
            "ifFn".to_string(), 
            Box::new(CompilationEngine::if_compiler)
        );
        list_func.insert(
            "classFn".to_string(),
            Box::new(CompilationEngine::class_compiler),
        );
        list_func.insert(
            "classvardecFn".to_string(),
            Box::new(CompilationEngine::class_var_dec_compiler),
        );
        list_func.insert(
            "subroutinedecFn".to_string(),
            Box::new(CompilationEngine::subroutine_dec_compiler),
        );
        list_func.insert(
            "parameterlistFn".to_string(),
            Box::new(CompilationEngine::parameter_list_compiler),
        );
        list_func.insert(
            "subroutinebodyFn".to_string(),
            Box::new(CompilationEngine::subroutine_body_compiler),
        );
        list_func.insert(
            "subroutinecallFn".to_string(),
            Box::new(CompilationEngine::subroutine_call_compiler),
        );
        #[rustfmt::skip]
        list_func.insert(
            "doFn".to_string(), 
            Box::new(CompilationEngine::do_compiler)
        );
        list_func.insert(
            "vardecFn".to_string(),
            Box::new(CompilationEngine::var_dec_compiler),
        );
        list_func.insert(
            "returnFn".to_string(),
            Box::new(CompilationEngine::return_compiler),
        );
        Self {
            list_func,
            pointer: 0,
            stack_compiler: Vec::new(),
            stack_state: Vec::new(),
        }
    }
}

impl StackCompiler {
    pub fn push(&mut self, function: String) {
        self.stack_compiler.push(function);
        self.stack_state.push(1);
        self.pointer = self.stack_compiler.len() - 1;
    }
    pub fn pop(&mut self) {
        if self.stack_compiler.len() == 0 { panic!("No fucntions in stack"); }        
        self.stack_compiler.pop();
        self.stack_state.pop();
        self.pointer = self.stack_compiler.len() - 1;
    }
    pub fn get(&mut self) -> (&mut CompilerFunc, &mut i8) {
        let key = &self.stack_compiler[self.pointer];
        (
            self.list_func.get_mut(key).unwrap(),
            &mut self.stack_state[self.pointer],
        )
    }
    pub fn is_empty(&self) -> bool {
        self.stack_state.is_empty()
    }
}
