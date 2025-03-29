#[derive(Debug, Clone)]
pub struct Program {
    pub includes: Vec<Statement>,
    pub functions: Vec<Function>,
}
#[derive(Debug, Clone)]
pub struct Function {
    pub return_type: Type,
    pub name: String,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Float,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Include(String),
    Declare(Type, String, Expr),
    Assign(String, Expr),
    Return(Expr),
    Print(Vec<Expr>),
    Scan(Vec<Expr>),
}

#[derive(Debug, Clone)]
pub enum Expr {
    IntLiteral(i32),
    FloatLiteral(f32),
    Variable(String),
    StringLiteral(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum PrintArg {
    String(String),
    Identifier(String),
}