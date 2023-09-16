use crate::prelude::*;
pub struct Grammar;

impl ProgramStructure for Grammar {
    fn class_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)> {
        // Handling "class".
        if *state == 1i8 {
            if kind == &TokenKind::Keyword && token == "class" {
                Ok((token.to_string(), "keyword".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Handling Class Name.
        else if *state == 2i8 {
            if kind == &TokenKind::Identifier {
                Ok((token.to_string(), "identifier".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Handling "{".
        else if *state == 3i8 {
            if kind == &TokenKind::Symbol && token == "{" {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Second index Tuple "Fn" Indicate to Return a Function "ClassVarDec".
        else if *state == 4i8 {
            if ["static", "field"].contains(&token) {
                Ok(("ClassVarDec".to_string(), "Fn".to_string()))
            } else {
                Ok(("SafeIterate".to_string(), "Iterate".to_string()))
            }
        }
        // Second index Tuple "Fn" Indicate to Return a Function "SubroutineDec".
        else if *state == 5i8 {
            if ["constructor", "method", "function"].contains(&token) {
                Ok(("SubroutineDec".to_string(), "Fn".to_string()))
            } else {
                Ok(("SafeIterate".to_string(), "Iterate".to_string()))
            }
        }
        // Handling "}".
        else if *state == 6i8 {
            *state = 0;
            if kind == &TokenKind::Symbol && token == "}" {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unrecognized".to_string(),
                ))
            }
        }
        // "SafePop", "Pop" Mean Pop Current Function "class" on Stack Without Changing Current Token to Next Token.
        else {
            Err(ParsingError::UnexpectedToken(
                "Token unrecognized".to_string(),
            ))
        }
    }

    fn class_var_dec_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)> {
        // Handling "static", "field".
        if *state == 1i8 {
            let approved = ["static", "field"];
            if kind == &TokenKind::Keyword && approved.contains(&token) {
                Ok((token.to_string(), "keyword".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Handling Type of Variable.
        else if *state == 2i8 {
            let approved = ["char", "int", "boolean"];
            if kind == &TokenKind::Identifier || approved.contains(&token) {
                Ok((token.to_string(), "keyword".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Handling Variable.
        else if *state == 3i8 {
            if kind == &TokenKind::Identifier {
                Ok((token.to_string(), "identifier".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Handling ";".
        else if *state == 4i8 {
            if kind == &TokenKind::Symbol && token == ";" {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // "SafePop", "Pop" Mean Pop Current Function "class" on Stack Without Changing Current Token to Next Token.
        else {
            *state = 0;
            Ok(("SafePop".to_string(), "Pop".to_string()))
        }
    }

    fn subroutine_dec_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)> {
        // Handling "constructor", "function", "method".
        if *state == 1i8 {
            let approved = ["constructor", "function", "method"];
            if kind == &TokenKind::Keyword && approved.contains(&token) {
                Ok((token.to_string(), "keyword".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Handling Type of Variable.
        else if *state == 2i8 {
            let approved = ["char", "int", "boolean", "void"];
            if kind == &TokenKind::Identifier || approved.contains(&token) {
                Ok((token.to_string(), "keyword".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Handling Variable.
        else if *state == 3i8 {
            if kind == &TokenKind::Identifier {
                Ok((token.to_string(), "identifier".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Handling "(".
        else if *state == 4i8 {
            if kind == &TokenKind::Symbol && token == "(" {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Second index Tuple "Fn" Indicate to Return a Function "ParameterList".
        else if *state == 5i8 {
            return Ok(("ParameterList".to_string(), "Fn".to_string()));
        }
        // Handling ")".
        else if *state == 6i8 {
            if kind == &TokenKind::Symbol && token == ")" {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Second index Tuple "Fn" Indicate to Return a Function "ParameterList".
        else if *state == 7i8 {
            Ok(("SubroutineBody".to_string(), "Fn".to_string()))
        }
        // "SafePop", "Pop" Mean Pop Current Function "class" on Stack Without Changing Current Token to Next Token.
        else {
            *state = 0;
            Ok(("SafePop".to_string(), "Pop".to_string()))
        }
    }

    fn parameter_list_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)> {
        if *state == 1i8 {
            Ok(("SafeIterate".to_string(), "Iterate".to_string()))
        }
        // Handling Type of Variable.
        else if *state == 2i8 {
            let approved = ["char", "int", "boolean"];
            if kind == &TokenKind::Identifier || approved.contains(&token) {
                Ok((token.to_string(), "keyword".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Handling Variable.
        else if *state == 3i8 {
            if kind == &TokenKind::Identifier {
                Ok((token.to_string(), "identifier".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // "SafePop", "Pop" Mean Pop Current Function "class" on Stack Without Changing Current Token to Next Token.
        else {
            *state = 0;
            Ok(("SafePop".to_string(), "Pop".to_string()))
        }
    }
    fn subroutine_body_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)> {
        // Handling "{".
        if *state == 1i8 {
            if kind == &TokenKind::Symbol && token == "{" {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Second index Tuple "Fn" Indicate to Return a Function "VarDec".
        else if *state == 2i8 {
            if token == "var" {
                Ok(("VarDec".to_string(), "Fn".to_string()))
            } else {
                Ok(("SafeIterate".to_string(), "Iterate".to_string()))
            }
        }
        // Second index Tuple "Fn" Indicate to Return a Function "Statements".
        else if *state == 3i8 {
            Ok(("Statements".to_string(), "Fn".to_string()))
        }
        // Handling "}".
        else if *state == 4i8 {
            if kind == &TokenKind::Symbol && token == "}" {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // "SafePop", "Pop" Mean Pop Current Function "class" on Stack Without Changing Current Token to Next Token.
        else {
            *state = 0;
            Ok(("SafePop".to_string(), "Pop".to_string()))
        }
    }

    fn var_dec_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)> {
        // Handling "var".
        if *state == 1i8 {
            if kind == &TokenKind::Keyword && token == "var" {
                Ok((token.to_string(), "keyword".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Handling Type of Variable.
        else if *state == 2i8 {
            let approved = ["char", "int", "boolean"];
            if kind == &TokenKind::Identifier || approved.contains(&token) {
                Ok((token.to_string(), "keyword".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Handling Variable.
        else if *state == 3i8 {
            if kind == &TokenKind::Identifier {
                Ok((token.to_string(), "identifier".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Handling ";".
        else if *state == 4i8 {
            if kind == &TokenKind::Symbol && token == ";" {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // "SafePop", "Pop" Mean Pop Current Function "class" on Stack Without Changing Current Token to Next Token.
        else {
            *state = 0;
            Ok(("SafePop".to_string(), "Pop".to_string()))
        }
    }
}

impl Statements for Grammar {
    fn statements_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)> {
        // Handling "let statement".
        if token == "let" && kind == &TokenKind::Keyword {
            Ok(("Let".to_string(), "Fn".to_string()))
        }
        // Handling "if statement".
        else if token == "if" && kind == &TokenKind::Keyword {
            Ok(("If".to_string(), "Fn".to_string()))
        }
        // Handling "while statement".
        else if token == "while" && kind == &TokenKind::Keyword {
            Ok(("While".to_string(), "Fn".to_string()))
        }
        // Handling "do statement".
        else if token == "do" && kind == &TokenKind::Keyword {
            Ok(("Do".to_string(), "Fn".to_string()))
        }
        // Handling "return statement".
        else if token == "return" && kind == &TokenKind::Keyword {
            Ok(("Return".to_string(), "Fn".to_string()))
        }
        // "SafePop", "Pop" Mean Pop Current Function "class" on Stack Without Changing Current Token to Next Token.
        else {
            *state = 0;
            Ok(("SafePop".to_string(), "Fn".to_string()))
        }
    }

    fn let_statement_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)> {
        // Handling "let".
        if *state == 1i8 {
            if kind == &TokenKind::Keyword && token == "let" {
                Ok((token.to_string(), "keyword".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Handling Variable Name.
        else if *state == 2i8 {
            if kind == &TokenKind::Identifier {
                Ok((token.to_string(), "identifier".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Handling Expression Inside Brackets.
        else if *state == 3i8 {
            if kind == &TokenKind::Symbol && token == "[" {
                Ok(("BracketsExpressions".to_string(), "Fn".to_string()))
            } else {
                Ok(("SafeIterate".to_string(), "Iterate".to_string()))
            }
        }
        // Handling "=".
        else if *state == 4i8 {
            if kind == &TokenKind::Symbol && token == "=" {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Handling Expression .
        else if *state == 5i8 {
            Ok(("Expression".to_string(), "Fn".to_string()))
        }
        // Handling "=".
        else if *state == 6i8 {
            if kind == &TokenKind::Symbol && token == ";" {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // "SafePop", "Pop" Mean Pop Current Function "class" on Stack Without Changing Current Token to Next Token.
        else {
            *state = 0;
            return Ok(("SafePop".to_string(), "Pop".to_string()));
        }
    }

    fn if_statement_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)> {
        // Handling "if".
        if *state == 1i8 {
            if kind == &TokenKind::Keyword && token == "if" {
                Ok((token.to_string(), "keyword".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Handling "(".
        else if *state == 2i8 {
            if kind == &TokenKind::Symbol && token == "(" {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Second index Tuple "Fn" Indicate to Return a Function "Expression".
        else if *state == 3i8 {
            Ok(("Expression".to_string(), "Fn".to_string()))
        }
        // Handling ")".
        else if *state == 4i8 {
            if kind == &TokenKind::Symbol && token == ")" {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Handling "{".
        else if *state == 5i8 {
            if kind == &TokenKind::Symbol && token == "{" {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Second index Tuple "Fn" Indicate to Return a Function "Statements".
        else if *state == 6i8 {
            Ok(("Statements".to_string(), "Fn".to_string()))
        }
        // Handling "}".
        else if *state == 7i8 {
            if kind == &TokenKind::Symbol && token == "}" {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        } else if *state == 8i8 {
            // handling else condition
            if kind == &TokenKind::Keyword && token == "else" {
                *state = 4;
                return Ok((token.to_string(), "keyword".to_string()));
            }
            *state = 0;
            return Ok(("SafePop".to_string(), "Pop".to_string()));
        }
        // "SafePop", "Pop" Mean Pop Current Function "class" on Stack Without Changing Current Token to Next Token.
        else {
            *state = 0;
            return Ok(("SafePop".to_string(), "Pop".to_string()));
        }
    }

    fn while_statement_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)> {
        // Handling "while".
        if *state == 1i8 {
            if kind == &TokenKind::Keyword && token == "while" {
                Ok((token.to_string(), "keyword".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Handling "(".
        else if *state == 2i8 {
            if kind == &TokenKind::Symbol && token == "(" {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Second index Tuple "Fn" Indicate to Return a Function "Expression".
        else if *state == 3i8 {
            Ok(("Expression".to_string(), "Fn".to_string()))
        }
        // Handling ")".
        else if *state == 4i8 {
            if kind == &TokenKind::Symbol && token == ")" {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Handling "{".
        else if *state == 5i8 {
            if kind == &TokenKind::Symbol && token == "{" {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Second index Tuple "Fn" Indicate to Return a Function "Statements".
        else if *state == 6i8 {
            Ok(("Statements".to_string(), "Fn".to_string()))
        }
        // Handling "}".
        else if *state == 7i8 {
            if kind == &TokenKind::Symbol && token == "}" {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // "SafePop", "Pop" Mean Pop Current Function "class" on Stack Without Changing Current Token to Next Token.
        else {
            *state = 0;
            Ok(("SafePop".to_string(), "Pop".to_string()))
        }
    }

    fn do_statement_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)> {
        // Handling "do".
        if *state == 1i8 {
            if kind == &TokenKind::Keyword && token == "do" {
                Ok((token.to_string(), "keyword".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Second index Tuple "Fn" Indicate to Return a Function "Statements".
        else if *state == 2i8 {
            Ok(("SubroutineCall".to_string(), "Fn".to_string()))
        }
        // Handling ";".
        else if *state == 3i8 {
            if kind == &TokenKind::Symbol && token == ";" {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // "SafePop", "Pop" Mean Pop Current Function "class" on Stack Without Changing Current Token to Next Token.
        else {
            *state = 0;
            Ok(("SafePop".to_string(), "Pop".to_string()))
        }
    }

    fn return_statement_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)> {
        if *state == 1 {
            Ok(("SafeIterate".to_string(), "Iterate".to_string()))
        }
        // Handling "return".
        else if *state == 2i8 {
            if kind == &TokenKind::Keyword && token == "return" {
                Ok((token.to_string(), "keyword".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Handling ";".
        else if *state == 3i8 {
            if kind == &TokenKind::Symbol && token == ";" {
                *state = i8::MIN;
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                *state = 2;
                Ok(("Expression".to_string(), "Fn".to_string()))
            }
        }
        // "SafePop", "Pop" Mean Pop Current Function "class" on Stack Without Changing Current Token to Next Token.
        else {
            *state = 0;
            Ok(("SafePop".to_string(), "Pop".to_string()))
        }
    }
}

impl Expressions for Grammar {
    fn expression_grammar(
        _token: &str,
        _kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)> {
        if *state == 1 {
            Ok(("SafeIterate".to_string(), "Iterate".to_string()))
        }
        // Second index Tuple "Fn" Indicate to Return a Function "Term".
        else if *state == 2i8 {
            Ok(("Term".to_string(), "Fn".to_string()))
        }
        // "SafePop", "Pop" Mean Pop Current Function "class" on Stack Without Changing Current Token to Next Token.
        else {
            *state = 0;
            Ok(("SafePop".to_string(), "Pop".to_string()))
        }
    }

    fn term_grammar(
        token: &str,
        context: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)> {
        // identifying Term.
        if *state == 1i8 {
            if ["~", "-"].contains(&token) {
                Ok((token.to_string(), "symbol".to_string()))
            } else if context == "." {
                Ok(("SubroutineCall".to_string(), "Fn".to_string()))
            } else if ["true", "false", "null", "this"].contains(&token) {
                Ok((token.to_string(), "keyword".to_string()))
            } else if kind == &TokenKind::StringVal {
                Ok((token.to_string(), "stringConstant".to_string()))
            } else if kind == &TokenKind::IntVal {
                Ok((token.to_string(), "integerConstant".to_string()))
            } else if kind == &TokenKind::Identifier {
                Ok((token.to_string(), "identifier".to_string()))
            } else if token == "(" {
                Ok(("ParenthesesExpressions".to_string(), "Fn".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        } else if *state == 2i8 {
            if token == "[" {
                Ok(("BracketsExpressions".to_string(), "Fn".to_string()))
            } else {
                *state = 0;
                Ok(("SafePop".to_string(), "Pop".to_string()))
            }
        } else if *state == 3i8 {
            Ok(("Term".to_string(), "Fn".to_string()))
        }
        // "SafePop", "Pop" Mean Pop Current Function "Term" on Stack Without Changing Current Token to Next Token.
        else {
            *state = 0;
            Ok(("SafePop".to_string(), "Pop".to_string()))
        }
    }

    fn subroutine_call_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)> {
        if *state == 1i8 {
            Ok(("SafeIterate".to_string(), "Iterate".to_string()))
        }
        // Handling Variable.
        else if *state == 2i8 {
            if kind == &TokenKind::Identifier {
                Ok((token.to_string(), "identifier".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Handling Expression.
        else if *state == 3i8 {
            if kind == &TokenKind::Symbol && token == "(" {
                Ok(("ParenthesesExpressionsList".to_string(), "Fn".to_string()))
            } else if kind == &TokenKind::Symbol && token == "[" {
                Ok(("BracketsExpressionsList".to_string(), "Fn".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // "SafePop", "Pop" Mean Pop Current Function "class" on Stack Without Changing Current Token to Next Token.
        else {
            *state = 0;
            Ok(("SafePop".to_string(), "Pop".to_string()))
        }
    }

    fn expression_list_grammar(
        _token: &str,
        _kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)> {
        // Second index Tuple "Fn" Indicate to Return a Function "Expression".
        if *state == 1i8 {
            Ok(("SafeIterate".to_string(), "Iterate".to_string()))
        } else if *state == 2i8 {
            Ok(("Expression".to_string(), "Fn".to_string()))
        }
        // "SafePop", "Pop" Mean Pop Current Function "class" on Stack Without Changing Current Token to Next Token.
        else {
            *state = 0;
            Ok(("SafePop".to_string(), "Pop".to_string()))
        }
    }
}

impl OptionalExpressions for Grammar {
    fn parantheses_expressions_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)> {
        // Handling "(".
        if *state == 1i8 {
            if kind == &TokenKind::Symbol && token == "(" {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Second index Tuple "Fn" Indicate to Return a Function "Expression".
        else if *state == 2i8 {
            return Ok(("Expression".to_string(), "Fn".to_string()));
        }
        // Handling ")".
        else if *state == 3i8 {
            if kind == &TokenKind::Symbol && token == ")" {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // "SafePop", "Pop" Mean Pop Current Function "parantheses_expressions_compiler" on Stack Without Changing Current Token to Next Token.
        else {
            *state = 0;
            Ok(("SafePop".to_string(), "Pop".to_string()))
        }
    }

    fn brackets_expressions_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)> {
        // Handling "(".
        if *state == 1i8 {
            if kind == &TokenKind::Symbol && token == "[" {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Second index Tuple "Fn" Indicate to Return a Function "Expression".
        else if *state == 2i8 {
            return Ok(("Expression".to_string(), "Fn".to_string()));
        }
        // Handling ")".
        else if *state == 3i8 {
            if kind == &TokenKind::Symbol && token == "]" {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // "SafePop", "Pop" Mean Pop Current Function "parantheses_expressions_compiler" on Stack Without Changing Current Token to Next Token.
        else {
            *state = 0;
            Ok(("SafePop".to_string(), "Pop".to_string()))
        }
    }

    fn parantheses_expressions_list_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)> {
        // Handling "(".
        if *state == 1i8 {
            if kind == &TokenKind::Symbol && token == "(" {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Second index Tuple "Fn" Indicate to Return a Function "Expression".
        else if *state == 2i8 {
            return Ok(("ExpressionList".to_string(), "Fn".to_string()));
        }
        // Handling ")".
        else if *state == 3i8 {
            if kind == &TokenKind::Symbol && token == ")" {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // "SafePop", "Pop" Mean Pop Current Function "parantheses_expressions_compiler" on Stack Without Changing Current Token to Next Token.
        else {
            *state = 0;
            Ok(("SafePop".to_string(), "Pop".to_string()))
        }
    }

    fn brackets_expressions_list_grammar(
        token: &str,
        kind: &TokenKind,
        state: &mut i8,
    ) -> ResultParser<(String, String)> {
        // Handling "(".
        if *state == 1i8 {
            if kind == &TokenKind::Symbol && token == "[" {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // Second index Tuple "Fn" Indicate to Return a Function "Expression".
        else if *state == 2i8 {
            return Ok(("ExpressionList".to_string(), "Fn".to_string()));
        }
        // Handling ")".
        else if *state == 3i8 {
            if kind == &TokenKind::Symbol && token == "]" {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Token unreconized".to_string(),
                ))
            }
        }
        // "SafePop", "Pop" Mean Pop Current Function "parantheses_expressions_compiler" on Stack Without Changing Current Token to Next Token.
        else {
            *state = 0;
            Ok(("SafePop".to_string(), "Pop".to_string()))
        }
    }
}
