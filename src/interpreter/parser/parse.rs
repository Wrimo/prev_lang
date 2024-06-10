use super::expr::{Expression, ExpressionType};
use super::parsing_types::{Token, TokenType};
use super::statement::{Program, Statement, StatementType};

pub struct Parser {
    current_token: Token,
    tokens: Vec<Token>,
    index: usize,
    prog: Program,
    stat: Statement,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            current_token: tokens[0].clone(),
            tokens: tokens,
            index: 0,
            prog: Program::new(),
            stat: Statement::new(),
        }
    }

    pub fn run(&mut self) -> &Program {
        self.body();
        return &self.prog;
    }

    fn error_missing_token(&self, t: TokenType) {
        eprintln!(
            "line {}: expected {:?} got {:?}",
            self.current_token.line + 1,
            t,
            self.current_token.token_type
        );
        assert!(false);
    }

    fn error_custom(&self, msg: &str) {
        eprintln!("line {}: {}", self.current_token.line, msg);
        assert!(false);
    }

    fn next_token(&mut self) -> Token {
        // moves to next token and returns previous
        self.index += 1;
        if self.index >= self.tokens.len() {
            self.current_token = Token {
                token_type: TokenType::NONE,
                line: self.current_token.line,
            };
            return self.current_token.clone();
        }
        self.current_token = self.tokens[self.index].clone();
        return self.tokens[self.index - 1].clone();
    }

    fn ahead(&self, i: usize) -> Token {
        self.tokens[self.index + i].clone()
    }

    fn accept(&mut self, t: TokenType) -> bool {
        if self.current_token.equals(t) {
            self.next_token();
            return true;
        }
        return false;
    }

    fn expect(&mut self, t: TokenType) -> bool {
        if self.accept(t.clone()) {
            return true;
        }
        self.error_missing_token(t);
        return false;
    }

    fn expect_identifier(&mut self) -> Option<String> {
        if self.current_token.equals(TokenType::IDENTIFIER(String::from(""))) {
            let t = self.current_token.clone();
            self.next_token();
            if let TokenType::IDENTIFIER(s) = t.token_type {
                return Some(s);
            }
            return None;
        }
        self.error_missing_token(TokenType::IDENTIFIER(String::from("")));
        None
    }

    fn body(&mut self) {
        loop {
            self.statement();
            self.expect(TokenType::NEWLINE);

            match self.stat.statement_type {
                StatementType::BEGIN => self.prog.begin = Some(self.stat.clone()),
                StatementType::EXPECT => self.prog.expect = Some(self.stat.clone()),
                _ => self.prog.add(self.stat.clone()),
            }
            self.stat.reset();

            if self.accept(TokenType::NONE) {
                break;
            }
        }
    }

    fn code_block(&mut self) -> Vec<Statement> {
        let old_stat = self.stat.clone();
        let mut code_block: Vec<Statement> = Vec::new();
        self.expect(TokenType::LBRACKET);
        self.expect(TokenType::NEWLINE);
        loop {
            self.statement();
            self.expect(TokenType::NEWLINE);
            code_block.push(self.stat.clone());
            self.stat.reset();
            if self.accept(TokenType::RBRACKET) {
                break;
            }
        }
        self.stat = old_stat;
        return code_block;
    }

    fn expr(&mut self) -> Box<Expression> {
        let mut lhs = self.expr_comp();
        while self.current_token.equals(TokenType::AND) || self.current_token.equals(TokenType::OR) {
            let old = self.next_token();
            let rhs = self.expr_comp();
            match old.token_type {
                TokenType::AND => lhs = Expression::new(ExpressionType::AND, Some(lhs), Some(rhs)),
                TokenType::OR => lhs = Expression::new(ExpressionType::OR, Some(lhs), Some(rhs)),
                _ => {}
            }
        }
        return lhs;
    }

    fn expr_comp(&mut self) -> Box<Expression> {
        let mut lhs = self.epxr_add();
        while self.current_token.equals(TokenType::GETHANOP)
            || self.current_token.equals(TokenType::GTHANOP)
            || self.current_token.equals(TokenType::EQUALOP)
            || self.current_token.equals(TokenType::NOTEQUALOP)
            || self.current_token.equals(TokenType::LTHANOP)
            || self.current_token.equals(TokenType::LETHANOP)
        {
            let old = self.next_token();
            let rhs = self.epxr_add();
            match old.token_type {
                TokenType::GETHANOP => lhs = Expression::new(ExpressionType::GTHE, Some(lhs), Some(rhs)),
                TokenType::GTHANOP => lhs = Expression::new(ExpressionType::GTH, Some(lhs), Some(rhs)),
                TokenType::EQUALOP => lhs = Expression::new(ExpressionType::EQU, Some(lhs), Some(rhs)),
                TokenType::NOTEQUALOP => lhs = Expression::new(ExpressionType::NEQU, Some(lhs), Some(rhs)),
                TokenType::LTHANOP => lhs = Expression::new(ExpressionType::LTH, Some(lhs), Some(rhs)),
                TokenType::LETHANOP => lhs = Expression::new(ExpressionType::LTHE, Some(lhs), Some(rhs)),

                _ => {}
            }
        }
        return lhs;
    }

    fn epxr_add(&mut self) -> Box<Expression> {
        let mut lhs = self.expr_mul();
        while self.current_token.equals(TokenType::ADDOP) || self.current_token.equals(TokenType::SUBOP) {
            let old = self.next_token();
            let rhs = self.expr_mul();

            match old.token_type {
                TokenType::ADDOP => lhs = Expression::new(ExpressionType::ADD, Some(lhs), Some(rhs)),
                TokenType::SUBOP => lhs = Expression::new(ExpressionType::SUB, Some(lhs), Some(rhs)),

                _ => {}
            }
        }
        return lhs;
    }

    fn expr_mul(&mut self) -> Box<Expression> {
        let mut lhs = self.expr_expo();
        while self.current_token.equals(TokenType::MULOP)
            || self.current_token.equals(TokenType::DIVOP)
            || self.current_token.equals(TokenType::MODOP)
        {
            let old = self.next_token();
            let rhs = self.expr_expo();
            match old.token_type {
                TokenType::MULOP => lhs = Expression::new(ExpressionType::MUL, Some(lhs), Some(rhs)),
                TokenType::DIVOP => lhs = Expression::new(ExpressionType::DIV, Some(lhs), Some(rhs)),
                TokenType::MODOP => lhs = Expression::new(ExpressionType::MOD, Some(lhs), Some(rhs)),

                _ => {}
            }
        }
        return lhs;
    }

    fn expr_expo(&mut self) -> Box<Expression> {
        let mut lhs = self.unary_fact();
        while self.accept(TokenType::EXPONENT) {
            let rhs = self.unary_fact();
            lhs = Expression::new(ExpressionType::EXPONENT, Some(lhs), Some(rhs));
        }
        return lhs;
    }

    fn unary_fact(&mut self) -> Box<Expression> {
        if self.accept(TokenType::NOT) {
            return Expression::new(ExpressionType::NOT, Some(self.factor()), None);
        } else if self.accept(TokenType::FACTORIAL) {
            return Expression::new(ExpressionType::FACTORIAL, Some(self.factor()), None);
        } else if self.accept(TokenType::SUBOP) {
            return Expression::new(ExpressionType::UMIN, Some(self.factor()), None);
        } else if self.accept(TokenType::VERTICALBAR) {
            return Expression::new(ExpressionType::ABS, Some(self.factor()), None);
        } else if self.accept(TokenType::PREV) {
            return Expression::new(ExpressionType::PREV(self.expect_identifier().unwrap()), None, None);
        }
        return self.accessor_factor();
    }

    fn accessor_factor(&mut self) -> Box<Expression> {
        let mut lhs = self.factor();
        while self.accept(TokenType::ACCESSOR) {
            let rhs = self.unary_fact();
            if !matches!(rhs.exp_type, ExpressionType::IDENTIFIER(_)) && !matches!(lhs.exp_type, ExpressionType::IDENTIFIER(_)) {
                self.error_custom("one side of accessing must be an identifier")
            }
            lhs = Expression::new(ExpressionType::ACCESSOR, Some(lhs), Some(rhs));
        }
        return lhs;
    }

    fn factor(&mut self) -> Box<Expression> {
        match self.next_token().token_type {
            TokenType::IDENTIFIER(s) => Expression::new(ExpressionType::IDENTIFIER(s), None, None),
            TokenType::INTEGER(x) => Expression::new(ExpressionType::INTEGER(x), None, None),
            TokenType::FLOAT(x) => Expression::new(ExpressionType::FLOAT(x), None, None),
            TokenType::TRUE => Expression::new(ExpressionType::BOOL(true), None, None),
            TokenType::FALSE => Expression::new(ExpressionType::BOOL(false), None, None),

            TokenType::LPAREN => {
                let exp = self.expr();
                self.expect(TokenType::RPAREN);
                return exp;
            }

            _ => {
                self.error_custom(format!("expression error for token {:?}", self.current_token).as_str());
                return Expression::new(ExpressionType::NONE, None, None);
            }
        }
    }

    fn statement(&mut self) {
        self.stat.reset();
        if self.current_token.equals(TokenType::IDENTIFIER(String::from(""))) && self.ahead(1).equals(TokenType::ASSIGNMENT) {
            self.parse_stmt_assign();
        } else if self.accept(TokenType::BEGIN) {
            self.parse_stmt_begin();
        } else if self.accept(TokenType::EXPECT) {
            self.parse_stmt_expect();
        } else if self.accept(TokenType::REVEAL) {
            self.parse_stmt_reveal();
        } else if self.accept(TokenType::PRINT) {
            self.parse_stmt_print();
        } else if self.accept(TokenType::IF) {
            self.parse_stmt_if();
        } else {
            self.error_custom("expected statement");
        }
    }

    fn parse_stmt_assign(&mut self) {
        self.stat.set_type(StatementType::ASSIGN);

        match self.current_token.token_type.clone() {
            TokenType::IDENTIFIER(s) => {
                self.stat.var_name = Some(s);
                self.next_token();
            }
            _ => self.error_missing_token(TokenType::IDENTIFIER(String::from(""))),
        }

        self.expect(TokenType::ASSIGNMENT);
        self.stat.expr = Some(self.expr());
    }

    fn parse_stmt_begin(&mut self) {
        self.stat.set_type(StatementType::BEGIN);
        self.stat.code_block = Some(self.code_block());
    }

    fn parse_stmt_expect(&mut self) {
        self.stat.set_type(StatementType::EXPECT);
        self.stat.expr = Some(self.expr());
        self.stat.code_block = Some(self.code_block())
    }

    fn parse_stmt_reveal(&mut self) {
        self.stat.set_type(StatementType::REVEAL);
        self.stat.var_name = self.expect_identifier();
    }

    fn parse_stmt_print(&mut self) {
        self.stat.set_type(StatementType::PRINT);
        self.expect(TokenType::LPAREN);

        if self.accept(TokenType::RPAREN) {
            // no given expression; print()
            self.stat.expr = Some(Expression::new(ExpressionType::NONE, None, None));
            return;
        }
        self.stat.expr = Some(self.expr());

        while self.accept(TokenType::COMMA) {
            let expr = self.expr();
            self.stat.alt_exps.push(expr);
        }

        self.expect(TokenType::RPAREN);
    }

    fn parse_stmt_if(&mut self) {
        self.stat.set_type(StatementType::IF);
        self.stat.expr = Some(self.expr());
        self.stat.code_block = Some(self.code_block());

        while self.accept(TokenType::ELIF) {
            let exp = self.expr();
            let block = self.code_block();
            self.stat.alt_exps.push(exp);
            self.stat.alt_code_blocks.push(block);
        }

        if self.accept(TokenType::ELSE) {
            let block = self.code_block();
            self.stat.alt_code_blocks.push(block);
        }
    }
}