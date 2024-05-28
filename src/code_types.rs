#[derive(Clone, Debug, PartialEq)]
pub enum StatementType {
    NONE,
    PRINT,
    REVEAL,
    ASSIGN,
    IF,
    ELSE,
    BEGIN,
    EXPECT,
}

#[derive(Clone, Debug)]
pub struct Statement {
    pub statement_type: StatementType,
    pub var_name: Option<String>,
    pub code_block: Option<Vec<Statement>>,
    pub expr: Option<Box<Expression>>,
    pub alt_code_blocks: Vec<Vec<Statement>>,
    pub alt_exps: Vec<Box<Expression>>,
}

pub struct Program {
    pub begin: Option<Statement>,
    pub expect: Option<Statement>,
    pub body: Vec<Statement>,
}

#[derive(Clone, Debug)]
pub enum Expression {
    ADD(Box<Expression>, Box<Expression>),
    SUB(Box<Expression>, Box<Expression>),
    MUL(Box<Expression>, Box<Expression>),
    DIV(Box<Expression>, Box<Expression>),
    MOD(Box<Expression>, Box<Expression>),
    EQU(Box<Expression>, Box<Expression>),
    NEQU(Box<Expression>, Box<Expression>),
    GTH(Box<Expression>, Box<Expression>),
    GTHE(Box<Expression>, Box<Expression>),
    LTH(Box<Expression>, Box<Expression>),
    LTHE(Box<Expression>, Box<Expression>),
    AND(Box<Expression>, Box<Expression>),
    OR(Box<Expression>, Box<Expression>),
    NOT(Box<Expression>),
    PREV(String),
    IDENTIFIER(String),
    BOOL(bool),
    INTEGER(i32),
    FLOAT(f32),
    NONE,
}

#[derive(Clone, Debug, PartialEq)]
pub enum VariableType {
    FLOAT(f32),
    INTEGER(i32),
    BOOL(bool),
    STRING(String),
}

impl VariableType {
    pub fn as_bool(&self) -> bool {
        match self {
            Self::FLOAT(x) => *x >= 1.0,
            Self::INTEGER(x) => *x >= 1,
            Self::BOOL(x) => *x,
            Self::STRING(x) => *x != "".to_string(),
        }
    }

    pub fn convert_bool(&mut self) -> Self {
        // if is a bool, converts it to an integer for expression eval
        match self {
            Self::BOOL(x) => *self = Self::INTEGER(if *x { 1 } else { 0 }),
            _ => {}
        }
        self.clone()
    }

    pub fn negate(&self) -> Self {
        VariableType::BOOL(!self.as_bool())
    }
}

impl StatementType {
    pub fn has_code_block(&self) -> bool {
        match self {
            StatementType::IF | StatementType::ELSE | StatementType::BEGIN | StatementType::EXPECT => true,
            _ => false,
        }
    }
}

impl Statement {
    pub fn reset(&mut self) {
        self.statement_type = StatementType::NONE;
        self.var_name = None;
        self.expr = None;
        self.code_block = None;
    }
}
