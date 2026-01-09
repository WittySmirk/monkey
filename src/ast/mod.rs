use crate::token;

pub trait Node {
    fn token_literal(&self) -> String {
        String::from("")
    }
}

pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut te: &str = "";
        match self {
            Statement::Let(_) => te = "LET",
            Statement::Return(_) => te = "RETURN",
        }

        write!(f, "{}", te)
    }
}

pub enum Expression {
    Identifier(Identifier),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let(s) => s.token_literal(),
            Statement::Return(s) => s.token_literal(),
        }
    }
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(i) => i.token_literal(),
        }
    }
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        }
        return String::from("");
    }
}

pub struct Identifier {
    pub token: token::Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

pub struct LetStatement {
    pub token: token::Token,
    pub name: Identifier,
    pub value: Expression,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

pub struct ReturnStatement {
    pub token: token::Token,
    pub expression: Expression,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
