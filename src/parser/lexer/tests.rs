
#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::parser::errors::SyntaxError;
    use super::super::tokens::Token;
    use crate::parser::lexer::Lexer;
    // things to test
    // 1) new
    // 2) ints
    // 3) floats
    // 4) strings
    // 5) chars
    // 6) an entire program
    // 7) keywords
    // 8) variables
    fn single_token(string: &str) -> Result<Token, SyntaxError> {
        let s = String::from(string);
        let mut lexer = Lexer::new(&s);
        lexer.next();
        lexer.next().unwrap()
    }
    #[test]
    fn string_token() {
        let result = single_token("\"string\\\"sad\"");
        let expect = Token::String("string\"sad".to_string());
        if let Ok(token) = result {
            assert_eq!(token, expect)
        } else {
            panic!()
        }
    }
    #[test]
    fn highlight_last_token() {
        let mut lexer = Lexer::new("print(whils 34.34 '\\n' data \"asd\" if at )");
        let expect = "  |\n1 | print(whils 34.34 \'\\n\' data \"asd\" if at )\n  |                             ^^^^^\nSyntax Error: This is a string";
        for result in &mut lexer {
            if let Ok(Token::String(_)) = result {
                break;
            }
        }
        assert_eq!(
            lexer
                .highlight_last_token("This is a string")
                .simple_display(),
            expect
        );
    }

    #[test]
    fn char_token() {
        let result = single_token("'a'");
        let expect = Token::Char("a".to_string());
        if let Ok(token) = result {
            assert_eq!(token, expect,)
        } else {
            panic!()
        }
    }
    #[test]
    fn scape_char() {
        let result = single_token("'\\\\'"); // equivalente to lemur '\\'
        let expect = Token::Char("\\".to_string());
        if let Ok(token) = result {
            assert_eq!(token, expect,)
        } else {
            panic!("Got {:?}", result)
        }
    }
    #[test]
    fn variable_token() {
        let result = single_token("variable_123");
        let expect = Token::Variable("variable_123".to_string());
        if let Ok(token) = result {
            assert_eq!(token, expect,)
        } else {
            panic!()
        }
    }
    #[test]
    fn integer_token() {
        let result = single_token("1234567890");
        let expect = Token::Integer("1234567890".parse().unwrap());
        if let Ok(token) = result {
            assert_eq!(token, expect,)
        } else {
            panic!()
        }
    }
    #[test]
    fn float_token0() {
        let result = single_token("1.234e-304-");
        let expect = Token::Float("1.234e-304".parse().unwrap());
        if let Ok(token) = result {
            assert_eq!(token, expect,)
        } else {
            panic!()
        }
    }
    #[test]
    fn float_token1() {
        let result = single_token("987e-654e");
        let expect = Token::Float("987e-654".parse().unwrap());
        if let Ok(token) = result {
            assert_eq!(token, expect,)
        } else {
            panic!()
        }
    }
    #[test]
    fn float_token2() {
        let result = single_token("124e+304+");
        let expect = Token::Float("124e+304".parse().unwrap());
        if let Ok(token) = result {
            assert_eq!(token, expect,)
        } else {
            panic!()
        }
    }
    #[test]
    fn float_token3() {
        let result = single_token("13.0987654321_");
        let expect = Token::Float("13.0987654321".parse().unwrap());
        if let Ok(token) = result {
            assert_eq!(token, expect,)
        } else {
            panic!()
        }
    }
    #[test]
    fn float_token4() {
        let result = single_token("124E-34");
        let expect = Token::Float("124e-34".parse().unwrap());
        if let Ok(token) = result {
            assert_eq!(token, expect,)
        } else {
            panic!()
        }
    }
    #[test]
    fn float_token5() {
        let result = single_token("13e34");
        let expect = Token::Float("13e+34".parse().unwrap());
        if let Ok(token) = result {
            assert_eq!(token, expect,)
        } else {
            panic!()
        }
    }
    #[test]
    fn float_token6() {
        let result = single_token("13E44");
        let expect = Token::Float("13e+44".parse().unwrap());
        if let Ok(token) = result {
            assert_eq!(token, expect,)
        } else {
            panic!()
        }
    }
    #[test]
    fn complex_token0() {
        let result = single_token("136i");
        let expect = Token::Complex("136".parse().unwrap());
        if let Ok(token) = result {
            assert_eq!(token, expect,)
        } else {
            panic!()
        }
    }
    #[test]
    fn complex_token1() {
        let result = single_token("13.09876i");
        let expect = Token::Complex("13.09876".parse().unwrap());
        if let Ok(token) = result {
            assert_eq!(token, expect,)
        } else {
            panic!()
        }
    }
    #[test]
    fn complex_token2() {
        let result = single_token("213e3i");
        let expect = Token::Complex("213e+3".parse().unwrap());
        if let Ok(token) = result {
            assert_eq!(token, expect,)
        } else {
            panic!()
        }
    }
    #[test]
    fn complex_token3() {
        let result = single_token("21.3e3i");
        let expect = Token::Complex("21.3e+3".parse().unwrap());
        if let Ok(token) = result {
            assert_eq!(token, expect,)
        } else {
            panic!()
        }
    }
    #[test]
    fn symbol_token0() {
        let result = single_token("¢∞¬÷654");
        let expect = Token::Symbol("¢∞¬÷".to_string());
        if let Ok(token) = result {
            assert_eq!(token, expect,)
        } else {
            panic!()
        }
    }
    #[test]
    fn symbol_token1() {
        let result = single_token("||");
        let expect = Token::Symbol("||".to_string());
        if let Ok(token) = result {
            assert_eq!(token, expect)
        } else {
            panic!()
        }
    }
    #[test]
    fn symbol_token2() {
        let result = single_token("&&2132");
        let expect = Token::Symbol("&&".to_string());
        if let Ok(token) = result {
            assert_eq!(token, expect,)
        } else {
            panic!()
        }
    }
}
