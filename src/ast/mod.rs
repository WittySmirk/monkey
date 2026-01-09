use crate::token;

pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut te: &str = "";
        match self {
            Statement::Let(_) => te = "LET",
            Statement::Return(_) => te = "RETURN",
            Statement::Expression(_) => te = "EXPRESSION",
        }

        write!(f, "{}", te)
    }
}

pub enum Expression {
    Identifier(Identifier),
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut te: &str = "";
        match self {
            Expression::Identifier(_) => te = "IDENTIFIER",
        }

        write!(f, "{}", te)
    }
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let(s) => s.token_literal(),
            Statement::Return(s) => s.token_literal(),
            Statement::Expression(s) => s.token_literal(),
        }
    }
    fn string(&self) -> String {
        match self {
            Statement::Let(s) => s.string(),
            Statement::Return(s) => s.string(),
            Statement::Expression(s) => s.string(),
        }
    }
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(i) => i.token_literal(),
        }
    }
    fn string(&self) -> String {
        match self {
            Expression::Identifier(i) => i.string(),
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
    fn string(&self) -> String {
        let mut buff: String = String::new();
        for statement in &self.statements {
            buff.push_str(statement.string().as_str());
        }
        buff
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

    fn string(&self) -> String {
        self.value.clone()
    }
}

pub struct LetStatement {
    pub token: token::Token,
    pub name: Identifier,
    pub value: Option<Expression>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        let mut buff: String = String::new();
        buff.push_str(self.token_literal().as_str());
        buff.push_str(" ");
        buff.push_str(self.name.string().as_str());
        buff.push_str(" = ");

        if self.value.is_some() {
            buff.push_str(self.value.as_ref().unwrap().string().as_str());
        }

        buff.push_str(";");

        buff
    }
}

pub struct ReturnStatement {
    pub token: token::Token,
    pub return_value: Option<Expression>,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        let mut buff: String = String::new();

        buff.push_str(self.token_literal().as_str());
        buff.push_str(" ");

        if self.return_value.is_some() {
            buff.push_str(self.return_value.as_ref().unwrap().string().as_str());
        }

        buff.push_str(";");

        buff
    }
}

pub struct ExpressionStatement {
    pub token: token::Token, // first token
    pub expression: Option<Expression>,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        if self.expression.is_some() {
            return String::from(self.expression.as_ref().unwrap().string().as_str());
        }
        return String::from("");
    }
}
