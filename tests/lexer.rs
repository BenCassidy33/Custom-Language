#[cfg(test)]
mod tests {
    use custom_language::lexer::{Lexer, Token, TokenType};

    fn run_lexer(source: &str) -> (Vec<Token>, Vec<TokenType>) {
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.generate_tokens();
        let kinds = tokens.iter().map(|t| t.kind.clone()).collect();

        (tokens.clone(), kinds)
    }

    #[test]
    fn test_variable_declaration() {
        let tokens = run_lexer("let x: int = 5");
        let expected = vec![
            TokenType::LET,
            TokenType::IDENTIFIER, // x
            TokenType::COLON,
            TokenType::IDENTIFIER, // int
            TokenType::EQUAL,
            TokenType::NUMBER,
        ];

        assert!(
            tokens.1 == expected,
            "\n[test_variable_declaration] Tokens did not match:\nExpected: {:?}\nActual:   {:?}\nTokens: {:?}",
            expected,
            tokens.1,
            tokens.0
        );
    }

    #[test]
    fn test_variable_declaration_with_special() {
        let tokens = run_lexer("let x_1: int = 5");
        let expected = vec![
            TokenType::LET,
            TokenType::IDENTIFIER, // x
            TokenType::COLON,
            TokenType::IDENTIFIER, // int
            TokenType::EQUAL,
            TokenType::NUMBER,
        ];

        assert!(
            tokens.1 == expected,
            "\n[test_variable_declaration] Tokens did not match:\nExpected: {:?}\nActual:   {:?}\nTokens: {:?}",
            expected,
            tokens.1,
            tokens.0
        );
    }

    #[test]
    fn test_function_declaration() {
        let tokens = run_lexer("fun add(a: int): int do end");
        let expected = vec![
            TokenType::FUN,
            TokenType::IDENTIFIER, // add
            TokenType::LEFT_PAREN,
            TokenType::IDENTIFIER, // a
            TokenType::COLON,
            TokenType::IDENTIFIER, // int
            TokenType::RIGHT_PAREN,
            TokenType::COLON,
            TokenType::IDENTIFIER, // int
            TokenType::DO,
            TokenType::END,
        ];

        assert!(
            tokens.1 == expected,
            "\n[test_variable_declaration] Tokens did not match:\nExpected: {:?}\nActual:   {:?}\nTokens: {:?}",
            expected,
            tokens.1,
            tokens.0
        );
    }

    #[test]
    fn test_struct_type_definition() {
        let code = r#"type Point = struct { x: int, y: int }"#;
        let tokens = run_lexer(code);
        let expected = vec![
            TokenType::TYPE,
            TokenType::IDENTIFIER, // Point
            TokenType::EQUAL,
            TokenType::STRUCT,
            TokenType::LEFT_BRACE,
            TokenType::IDENTIFIER, // x
            TokenType::COLON,
            TokenType::IDENTIFIER, // int
            TokenType::COMMA,
            TokenType::IDENTIFIER, // y
            TokenType::COLON,
            TokenType::IDENTIFIER, // int
            TokenType::RIGHT_BRACE,
        ];

        assert!(
            tokens.1 == expected,
            "\n[test_variable_declaration] Tokens did not match:\nExpected: {:?}\nActual:   {:?}\nTokens: {:?}",
            expected,
            tokens.1,
            tokens.0
        );
    }

    #[test]
    fn test_enum_definition() {
        let code = r#"type Color = enum { Red, Green }"#;
        let tokens = run_lexer(code);
        let expected = vec![
            TokenType::TYPE,
            TokenType::IDENTIFIER, // Color
            TokenType::EQUAL,
            TokenType::ENUM,
            TokenType::LEFT_BRACE,
            TokenType::IDENTIFIER, // Red
            TokenType::COMMA,
            TokenType::IDENTIFIER, // Green
            TokenType::RIGHT_BRACE,
        ];

        assert!(
            tokens.1 == expected,
            "\n[test_variable_declaration] Tokens did not match:\nExpected: {:?}\nActual:   {:?}\nTokens: {:?}",
            expected,
            tokens.1,
            tokens.0
        );
    }

    #[test]
    fn test_enum_definition_with_values() {
        let code = r#"type Color = enum : int { Red = 0, Green = 1 }"#;
        let tokens = run_lexer(code);
        let expected = vec![
            TokenType::TYPE,
            TokenType::IDENTIFIER, // Color
            TokenType::EQUAL,
            TokenType::ENUM,
            TokenType::COLON,
            TokenType::IDENTIFIER, // int
            TokenType::LEFT_BRACE,
            TokenType::IDENTIFIER, // Red
            TokenType::EQUAL,
            TokenType::NUMBER, // 0
            TokenType::COMMA,
            TokenType::IDENTIFIER, // Green
            TokenType::EQUAL,
            TokenType::NUMBER, // 1
            TokenType::RIGHT_BRACE,
        ];

        assert!(
            tokens.1 == expected,
            "\n[test_variable_declaration] Tokens did not match:\nExpected: {:?}\nActual:   {:?}\nTokens: {:?}",
            expected,
            tokens.1,
            tokens.0
        );
    }

    #[test]
    fn test_ducktype_interface() {
        let code = r#"type Info = ducktype { os: String, version: Float }"#;
        let tokens = run_lexer(code);
        let expected = vec![
            TokenType::TYPE,
            TokenType::IDENTIFIER, // Info
            TokenType::EQUAL,
            TokenType::DUCK_TYPE,
            TokenType::LEFT_BRACE,
            TokenType::IDENTIFIER, // os
            TokenType::COLON,
            TokenType::IDENTIFIER, // String
            TokenType::COMMA,
            TokenType::IDENTIFIER, // version
            TokenType::COLON,
            TokenType::IDENTIFIER, // Float
            TokenType::RIGHT_BRACE,
        ];

        assert!(
            tokens.1 == expected,
            "\n[test_variable_declaration] Tokens did not match:\nExpected: {:?}\nActual:   {:?}\nTokens: {:?}",
            expected,
            tokens.1,
            tokens.0
        );
    }
}
