use crate::errors::ParsingError;
use crate::prelude::*;
type Result<T> = std::result::Result<T, ParsingError>;
pub struct Grammar;

trait ProgramStructure {
    #[rustfmt::skip]
    fn class_grammar(
        &self, 
        tokenizer_: &JackTokenizer, 
        state: &mut i8
    )
        -> Result<(String, String)>;
        
    fn class_var_dec_grammar(
        &self,
        tokenizer_: &JackTokenizer,
        state: &mut i8,
    ) -> Result<(String, String)>;

    fn subroutine_dec_grammar(
        &self,
        tokenizer_: &JackTokenizer,
        state: &mut i8,
    ) -> Result<(String, String)>;

    fn parameter_list_grammar(
        &self,
        tokenizer_: &JackTokenizer,
        state: &mut i8,
    ) -> Result<(String, String)>;

    fn subroutine_body_grammar(
        &self,
        tokenizer_: &JackTokenizer,
        state: &mut i8,
    ) -> Result<(String, String)>;

    fn var_dec_grammar(
        &self,
        tokenizer_: &JackTokenizer,
        state: &mut i8,
    ) -> Result<(String, String)>;
}

impl ProgramStructure for Grammar {
    fn class_grammar(
        &self,
        tokenizer_: &JackTokenizer,
        state: &mut i8,
    ) -> Result<(String, String)> {
        let token = tokenizer_.tokens.get(0).unwrap();
        let kind = tokenizer_.token_kinds.get(0).unwrap();

        // Handling "class".
        if *state == 1i8 {
            let to_return = if kind == &TokenKind::Keyword && token == &"class".to_string() {
                Ok((token.to_string(), "keyword".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken)
            };
            return to_return;
        }

        // Handling Class Name.
        if *state == 2i8 {
            let to_return = if kind == &TokenKind::Identifier {
                Ok((token.to_string(), "identifier".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken)
            };
            return to_return;
        }

        // Handling "{".
        if *state == 3i8 {
            let to_return = if kind == &TokenKind::Symbol && token == &"{{".to_string() {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken)
            };
            return to_return;
        }

        // Second index Tuple "Fn" Indicate to Return a Function "ClassVarDec".
        if *state == 4i8 {
            return Ok(("ClassVarDec".to_string(), "Fn".to_string()));
        }

        // Second index Tuple "Fn" Indicate to Return a Function "SubroutineDec".
        if *state == 5i8 {
            return Ok(("SubroutineDec".to_string(), "Fn".to_string()));
        }

        // Handling "}".
        if *state == 6i8 {
            let to_return = if kind == &TokenKind::Symbol && token == &"}}".to_string() {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken)
            };
            return to_return;
        }
        // "Safe", "Pop" Mean Pop Current Function "class" on Stack Without Changing Current Token to Next Token.
        else {
            return Ok(("Safe".to_string(), "Pop".to_string()));
        }
    }

    fn class_var_dec_grammar(
        &self,
        tokenizer_: &JackTokenizer,
        state: &mut i8,
    ) -> Result<(String, String)> {
        let token = tokenizer_.tokens.get(0).unwrap();
        let kind = tokenizer_.token_kinds.get(0).unwrap();

        // Handling "static", "field".
        if *state == 1i8 {
            let approved = ["static", "field"];
            let to_return = if kind == &TokenKind::Keyword && approved.contains(&token.as_str()) {
                Ok((token.to_string(), "keyword".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken)
            };
            return to_return;
        }

        // Handling Type of Variable.
        if *state == 2i8 {
            let approved = ["char", "int", "boolean"];
            let to_return = if kind == &TokenKind::Keyword || approved.contains(&token.as_str()) {
                Ok((token.to_string(), "keyword".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken)
            };
            return to_return;
        }

        // Handling Variable.
        if *state == 3i8 {
            let to_return = if kind == &TokenKind::Identifier {
                Ok((token.to_string(), "identifier".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken)
            };
            return to_return;
        }

        // Handling ";".
        if *state == 4i8 {
            let to_return = if kind == &TokenKind::Symbol && token == &";".to_string() {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken)
            };
            return to_return;
        }
        // "Safe", "Pop" Mean Pop Current Function "class" on Stack Without Changing Current Token to Next Token.
        else {
            return Ok(("Safe".to_string(), "Pop".to_string()));
        }
    }

    fn subroutine_dec_grammar(
        &self,
        tokenizer_: &JackTokenizer,
        state: &mut i8,
    ) -> Result<(String, String)> {
        let token = tokenizer_.tokens.get(0).unwrap();
        let kind = tokenizer_.token_kinds.get(0).unwrap();

        // Handling "constructor", "function", "method".
        if *state == 1i8 {
            let approved = ["constructor", "function", "method"];
            let to_return = if kind == &TokenKind::Keyword && approved.contains(&token.as_str()) {
                Ok((token.to_string(), "keyword".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken)
            };
            return to_return;
        }

        // Handling Type of Variable.
        if *state == 2i8 {
            let approved = ["char", "int", "boolean", "void"];
            let to_return = if kind == &TokenKind::Keyword || approved.contains(&token.as_str()) {
                Ok((token.to_string(), "keyword".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken)
            };
            return to_return;
        }

        // Handling Variable.
        if *state == 3i8 {
            let to_return = if kind == &TokenKind::Identifier {
                Ok((token.to_string(), "identifier".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken)
            };
            return to_return;
        }

        // Handling "(".
        if *state == 4i8 {
            let to_return = if kind == &TokenKind::Symbol && token == &"(".to_string() {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken)
            };
            return to_return;
        }

        // Second index Tuple "Fn" Indicate to Return a Function "ParameterList".
        if *state == 5i8 {
            return Ok(("ParameterList".to_string(), "Fn".to_string()));
        }

        // Handling ")".
        if *state == 6i8 {
            let to_return = if kind == &TokenKind::Symbol && token == &")".to_string() {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken)
            };
            return to_return;
        }

        // Second index Tuple "Fn" Indicate to Return a Function "ParameterList".
        if *state == 7i8 {
            return Ok(("SubroutineBody".to_string(), "Fn".to_string()));
        }
        // "Safe", "Pop" Mean Pop Current Function "class" on Stack Without Changing Current Token to Next Token.
        else {
            return Ok(("Safe".to_string(), "Pop".to_string()));
        }
    }

    fn parameter_list_grammar(
        &self,
        tokenizer_: &JackTokenizer,
        state: &mut i8,
    ) -> Result<(String, String)> {
        let token = tokenizer_.tokens.get(0).unwrap();
        let kind = tokenizer_.token_kinds.get(0).unwrap();

        // Handling Type of Variable.
        if *state == 1i8 {
            let approved = ["char", "int", "boolean"];
            let to_return = if kind == &TokenKind::Keyword || approved.contains(&token.as_str()) {
                Ok((token.to_string(), "keyword".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken)
            };
            return to_return;
        }

        // Handling Variable.
        if *state == 2i8 {
            let to_return = if kind == &TokenKind::Identifier {
                Ok((token.to_string(), "identifier".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken)
            };
            return to_return;
        }
        // "Safe", "Pop" Mean Pop Current Function "class" on Stack Without Changing Current Token to Next Token.
        else {
            return Ok(("Safe".to_string(), "Pop".to_string()));
        }
    }
    fn subroutine_body_grammar(
        &self,
        tokenizer_: &JackTokenizer,
        state: &mut i8,
    ) -> Result<(String, String)> {
        let token = tokenizer_.tokens.get(0).unwrap();
        let kind = tokenizer_.token_kinds.get(0).unwrap();

        // Handling "{".
        if *state == 1i8 {
            let to_return = if kind == &TokenKind::Symbol && token == &"{{".to_string() {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken)
            };
            return to_return;
        }

        // Second index Tuple "Fn" Indicate to Return a Function "VarDec".
        if *state == 2i8 {
            return Ok(("VarDec".to_string(), "Fn".to_string()));
        }

        // Second index Tuple "Fn" Indicate to Return a Function "Statements".
        if *state == 3i8 {
            return Ok(("Statements".to_string(), "Fn".to_string()));
        }

        // Handling "}".
        if *state == 4i8 {
            let to_return = if kind == &TokenKind::Symbol && token == &"}}".to_string() {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken)
            };
            return to_return;
        }
        // "Safe", "Pop" Mean Pop Current Function "class" on Stack Without Changing Current Token to Next Token.
        else {
            return Ok(("Safe".to_string(), "Pop".to_string()));
        }
    }

    fn var_dec_grammar(
        &self,
        tokenizer_: &JackTokenizer,
        state: &mut i8,
    ) -> Result<(String, String)> {
        let token = tokenizer_.tokens.get(0).unwrap();
        let kind = tokenizer_.token_kinds.get(0).unwrap();

        // Handling "var".
        if *state == 1i8 {
            let to_return = if kind == &TokenKind::Keyword && token == &"var".to_string() {
                Ok((token.to_string(), "keyword".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken)
            };
            return to_return;
        }

        // Handling Type of Variable.
        if *state == 2i8 {
            let approved = ["char", "int", "boolean"];
            let to_return = if kind == &TokenKind::Keyword || approved.contains(&token.as_str()) {
                Ok((token.to_string(), "keyword".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken)
            };
            return to_return;
        }

        // Handling Variable.
        if *state == 3i8 {
            let to_return = if kind == &TokenKind::Identifier {
                Ok((token.to_string(), "identifier".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken)
            };
            return to_return;
        }

        // Handling ";".
        if *state == 4i8 {
            let to_return = if kind == &TokenKind::Symbol && token == &";".to_string() {
                Ok((token.to_string(), "symbol".to_string()))
            } else {
                Err(ParsingError::UnexpectedToken)
            };
            return to_return;
        }
        // "Safe", "Pop" Mean Pop Current Function "class" on Stack Without Changing Current Token to Next Token.
        else {
            return Ok(("Safe".to_string(), "Pop".to_string()));
        }
    }
}
