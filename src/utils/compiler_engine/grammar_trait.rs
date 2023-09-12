use crate::prelude::*;

pub trait ProgramStructure {
    #[rustfmt::skip]
    fn class_grammar(
        &self, 
        tokenizer_: &JackTokenizer, 
        state: &mut i8
    )
        -> ResultParser<(String, String)>;
        
    fn class_var_dec_grammar(
        &self,
        tokenizer_: &JackTokenizer,
        state: &mut i8,
    ) -> ResultParser<(String, String)>;

    fn subroutine_dec_grammar(
        &self,
        tokenizer_: &JackTokenizer,
        state: &mut i8,
    ) -> ResultParser<(String, String)>;

    fn parameter_list_grammar(
        &self,
        tokenizer_: &JackTokenizer,
        state: &mut i8,
    ) -> ResultParser<(String, String)>;

    fn subroutine_body_grammar(
        &self,
        tokenizer_: &JackTokenizer,
        state: &mut i8,
    ) -> ResultParser<(String, String)>;

    fn var_dec_grammar(
        &self,
        tokenizer_: &JackTokenizer,
        state: &mut i8,
    ) -> ResultParser<(String, String)>;
}

pub trait Statements {
    fn let_statement_grammar(
        &self,
        tokenizer_: &JackTokenizer,
        state: &mut i8,
    ) -> ResultParser<(String, String)>;
    
    fn if_statement_grammar(
        &self,
        tokenizer_: &JackTokenizer,
        state: &mut i8,
    ) -> ResultParser<(String, String)>;
    
    fn while_statement_grammar(
        &self,
        tokenizer_: &JackTokenizer,
        state: &mut i8,
    ) -> ResultParser<(String, String)>;
    
    fn do_statement_grammar(
        &self,
        tokenizer_: &JackTokenizer,
        state: &mut i8,
    ) -> ResultParser<(String, String)>;
    
    fn return_statement_grammar(
        &self,
        tokenizer_: &JackTokenizer,
        state: &mut i8,
    ) -> ResultParser<(String, String)>;
}

pub trait Expressions {
    fn expression_grammar(
        &self,
        tokenizer_: &JackTokenizer,
        state: &mut i8,
    ) -> ResultParser<(String, String)>;
    
    fn term_grammar(
        &self,
        tokenizer_: &JackTokenizer,
        state: &mut i8,
    ) -> ResultParser<(String, String)>;
    
    fn subroutine_call_grammar(
        &self,
        tokenizer_: &JackTokenizer,
        state: &mut i8,
    ) -> ResultParser<(String, String)>;
    
    fn expression_list_grammar(
        &self,
        tokenizer_: &JackTokenizer,
        state: &mut i8,
    ) -> ResultParser<(String, String)>;
}