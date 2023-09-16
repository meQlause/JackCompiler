use crate::prelude::*;

pub trait ProgramStructure {
    #[rustfmt::skip]
    fn class_grammar(
        token: &str,
        kind: &TokenKind, 
        state: &mut i8
    )
        -> ResultParser<(String, String)>;
        
    fn class_var_dec_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)>;

    fn subroutine_dec_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)>;

    fn parameter_list_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)>;

    fn subroutine_body_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)>;

    fn var_dec_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)>;
}

pub trait Statements {
    fn statements_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)>;

    fn let_statement_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)>;
    
    fn if_statement_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)>;
    
    fn while_statement_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)>;
    
    fn do_statement_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)>;
    
    fn return_statement_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)>;
}

pub trait Expressions {
    fn expression_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)>;
    
    fn term_grammar(
        token: &str,
        context: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)>;
    
    fn subroutine_call_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)>;
    
    fn expression_list_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)>;
}

pub trait OptionalExpressions {
    fn parantheses_expressions_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)>;
    
    fn brackets_expressions_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)>;

    fn parantheses_expressions_list_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)>;
    
    fn brackets_expressions_list_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)>;
}