use crate::{
    ast::{
        ast::{Item, Program},
        expressions::ExpressionType,
        statements::Statement,
    },
    lexer::{Token, TokenType},
};

pub struct Ast {
    program: Program,
    tokens: Vec<Token>,
}

fn creates_grouping(token: &Token) -> bool {
    return match token.kind {
        TokenType::IF
        | TokenType::FOR
        | TokenType::WHILE
        | TokenType::FUN
        | TokenType::LEFT_BRACE => true,
        _ => false,
    };
}

impl Ast {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            program: Program {
                start: 0,
                end: tokens[tokens.len() - 1].end,
                body: Vec::new(),
            },

            tokens,
        }
    }

    pub fn generate_ast(&mut self) -> Program {
        todo!()
    }

    /// for use with expressions that end with the 'end' keyword. Also filters out newlines
    pub fn consume_group(&mut self) -> Vec<Token> {
        let mut end_count = (0, false);
        let mut tokens = Vec::new();
        let mut idx = 0;

        while idx < self.tokens.len() {
            if self.tokens.is_empty() {
                break;
            }

            let token = &self.tokens[idx];
            print!("{}", token.literal);

            if creates_grouping(token) {
                end_count.0 += 1;

                if !end_count.1 {
                    end_count.1 = true;
                }

                idx += 1;
                continue;
            }

            if token.kind == TokenType::END || token.kind == TokenType::RIGHT_BRACE {
                end_count.0 -= 1;
            }

            if end_count.0 == 0 && end_count.1 {
                tokens = self.tokens.drain(0..=idx).collect()
            }

            idx += 1;
        }

        tokens
        //todo!()
    }

    pub fn det_expr_type(first_token: &Token) -> fn(&[Token]) -> Item {
        match first_token.kind {
            TokenType::IF => Generator::gen_if_statement,
            TokenType::FOR => Generator::gen_for_statement,
            TokenType::WHILE => Generator::gen_for_statement,

            TokenType::LET | TokenType::CONST => Generator::gen_var_decl,
            TokenType::FUN => Generator::gen_fun_decl,
            _ => todo!(),
        }
    }
}

pub struct Generator;

impl Generator {
    pub fn gen_if_statement(tokens: &[Token]) -> Item {
        todo!()
    }

    pub fn gen_while_statement(tokens: &[Token]) -> Item {
        todo!()
    }

    pub fn gen_for_statement(tokens: &[Token]) -> Item {
        todo!()
    }

    pub fn gen_fun_decl(tokens: &[Token]) -> Item {
        todo!()
    }

    pub fn gen_var_decl(tokens: &[Token]) -> Item {
        todo!()
    }
}
