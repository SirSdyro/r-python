use std::collections::HashMap;

pub type Name = String;

#[derive(Debug, Clone, PartialEq)]
pub enum EnvValue {
    Exp(Expression),
    Func(Function),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub value: Option<EnvValue>,
    pub kind: Type,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub params: Option<Vec<(Name, Type)>>,
    pub body: Box<Statement>,
    pub return_type: Type,
}

pub type Environment = HashMap<Name, Variable>;

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    TInteger,
    TBool,
    TReal,
    TString,
    TFunction,
    TList(Box<Type>),
    TTuple(Vec<Type>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    /* constants */
    CTrue,
    CFalse,
    CInt(i32),
    CReal(f64),
    CString(String),

    /* variable reference */
    Var(Name),

    /* function call */
    FuncCall(Name, Vec<Expression>),

    /* arithmetic expressions over numbers */
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Rmd(Box<Expression>, Box<Expression>),

    /* boolean expressions over booleans */
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Not(Box<Expression>),

    /* relational expressions over numbers */
    EQ(Box<Expression>, Box<Expression>),
    GT(Box<Expression>, Box<Expression>),
    LT(Box<Expression>, Box<Expression>),
    GTE(Box<Expression>, Box<Expression>),
    LTE(Box<Expression>, Box<Expression>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    VarDeclaration(Name),
    ValDeclaration(Name),
    Assignment(Name, Box<Expression>, Type),
    IfThenElse(Box<Expression>, Box<Statement>, Option<Box<Statement>>),
    While(Box<Expression>, Box<Statement>),
    Sequence(Box<Statement>, Box<Statement>),
    FuncDef(Name, Function, Type),
    Return(Name, Box<Expression>),
}
