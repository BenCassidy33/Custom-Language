use std::collections::HashSet;

use regex::Regex;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single Character Tokens
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    LEFT_BRACKET,
    RIGHT_BRACKET,
    COMMA,
    DOT,
    PLUS,
    MINUS,
    SEMICOLON,
    SLASH,
    STAR,
    COLON,
    SINGLE_QUOTE,
    DOUBLE_QUOTE,
    PIPE,

    // One or two character tokens
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords
    AND,
    IF,
    ELSE,
    TRUE,
    FALSE,
    TYPE,
    FUN,
    FOR,
    NIL,
    OR,
    PRINT,
    RETURN,
    SELF,
    LET,
    CONST,
    WHILE,
    STRUCT,
    ENUM,
    UNION,
    DUCK_TYPE,
    INTERFACE,
    CONTINUE,
    BREAK,
    DO,
    END,

    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenType,
    literal: String,
    start: usize,
    end: usize,
}

#[derive(Debug)]
pub struct Lexer {
    input: String,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input,
            tokens: Vec::new(),
        }
    }

    pub fn generate_tokens(&mut self) -> &Vec<Token> {
        #[rustfmt::skip]
        let mut regexes = vec![
            (r"\(", TokenType::LEFT_PAREN),
            (r"\)", TokenType::RIGHT_PAREN),
            (r"\{", TokenType::LEFT_BRACE),
            (r"\}", TokenType::RIGHT_BRACE),
            (r"\[", TokenType::LEFT_BRACKET),
            (r"\]", TokenType::RIGHT_BRACKET),

            (r",", TokenType::COMMA),
            (r"\.", TokenType::DOT),
            (r"\+", TokenType::PLUS),
            (r"-", TokenType::MINUS),
            (r";", TokenType::SEMICOLON),
            (r"\/", TokenType::SLASH),
            (r"\*", TokenType::STAR),
            (r":", TokenType::COLON),
            (r"'", TokenType::SINGLE_QUOTE),
            (r#"\""#, TokenType::DOUBLE_QUOTE),
            (r"\|", TokenType::PIPE),
            (r"!", TokenType::BANG),
            (r"!=", TokenType::BANG_EQUAL),
            (r"=", TokenType::EQUAL),
            (r"==", TokenType::EQUAL_EQUAL),
            (r">", TokenType::GREATER),
            (r">=", TokenType::GREATER_EQUAL),
            (r"<", TokenType::LESS),
            (r"<=", TokenType::LESS_EQUAL),

            (r"\band\b", TokenType::AND),
            (r"\bif\b", TokenType::IF),
            (r"\belse\b", TokenType::ELSE),
            (r"\btrue\b", TokenType::TRUE),
            (r"\bfalse\b", TokenType::FALSE),
            (r"\btype\b", TokenType::TYPE),
            (r"\bfun\b", TokenType::FUN),
            (r"\bfor\b", TokenType::FOR),
            (r"\bnil\b", TokenType::NIL),
            (r"\bor\b", TokenType::OR),
            (r"\bprint\b", TokenType::PRINT),
            (r"\breturn\b", TokenType::RETURN),
            (r"\bself\b", TokenType::SELF),
            (r"\blet\b", TokenType::LET),
            (r"\bconst\b", TokenType::CONST),
            (r"\bwhile\b", TokenType::WHILE),
            (r"\bstruct\b", TokenType::STRUCT),
            (r"\benum\b", TokenType::ENUM),
            (r"\bunion\b", TokenType::UNION),
            (r"\bducktype\b", TokenType::DUCK_TYPE),
            (r"\binterface\b", TokenType::INTERFACE),
            (r"\bcontinue\b", TokenType::CONTINUE),
            (r"\bbreak\b", TokenType::BREAK),
            (r"\bdo\b", TokenType::DO),
            (r"\bend\b", TokenType::END),


            (r"\b[0-9]+\b", TokenType::NUMBER),
            (r"\b[a-zA-Z_][a-zA-Z0-9_]*\b", TokenType::IDENTIFIER)
        ];

        #[rustfmt::skip]
        let keywords: HashSet<&str> = [
            "let", "fun", "type", "struct", "enum", "if", "else", "true", "false",
            "return", "do", "end", "while", "for", "interface", "ducktype", "union",
            "const", "and", "or", "print", "self", "continue", "break", "nil"
        ].iter().copied().collect();

        for (pattern, kind) in regexes {
            let re = Regex::new(&format!(r"({})", pattern)).unwrap();

            for caps in re.captures_iter(&self.input) {
                if let Some(m) = caps.get(0) {
                    if kind == TokenType::IDENTIFIER && keywords.contains(m.as_str()) {
                        continue;
                    }

                    if kind == TokenType::TYPE && m.as_str().contains("ducktype") {
                        continue;
                    }

                    self.tokens.push(Token {
                        kind: kind.clone(),
                        literal: m.as_str().to_string(),
                        start: m.start(),
                        end: m.end(),
                    })
                }
            }
        }

        self.tokens.sort_by(|a, b| a.start.cmp(&b.start));
        &self.tokens
    }
}
