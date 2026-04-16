#[derive(Debug, Clone)]
pub enum AstNode {
    Number(i64),
    Float(f64),
    Boolean(bool),
    StringLiteral(String),
    Identifier(String),
    BinaryOp {
        left: Box<AstNode>,
        op: Operator,
        right: Box<AstNode>,
    },
    VarDecl {
        name: String,
        var_type: VarType,
    },
    ConstDecl {
        name: String,
        var_type: VarType,
    },
    Assignment {
        name: String,
        value: Box<AstNode>,
    },
    ToStr(Box<AstNode>),
    ToInt(Box<AstNode>),
    Printf(Box<AstNode>),
    Program(Vec<AstNode>),
}

#[derive(Debug, Clone)]
pub enum Operator {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    // Comparison
    Equal,
    NotEqual,
    LessThanOrEqual,
    GreaterThanOrEqual,
    LessThan,
    GreaterThan,
    // Logical
    And,
    Or,
    Xor,
    Nand,
}

#[derive(Debug, Clone)]
pub enum VarType {
    Int,
    Bool,
    Str,
    Float,
}