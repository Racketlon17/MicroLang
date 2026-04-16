use crate::ast::{AstNode, Operator, VarType};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    None,
}

pub struct Interpreter {
    variables: HashMap<String, Value>,
    constants: HashSet<String>,
    variable_types: HashMap<String, VarType>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            constants: HashSet::new(),
            variable_types: HashMap::new(),
        }
    }
    
    pub fn eval(&mut self, node: &AstNode) -> Value {
        match node {
            AstNode::Number(n) => Value::Int(*n),
            
            AstNode::Float(f) => Value::Float(*f),
            
            AstNode::Boolean(b) => Value::Bool(*b),
            
            AstNode::StringLiteral(s) => Value::String(s.clone()),
            
            AstNode::Identifier(name) => {
                self.variables.get(name)
                    .cloned()
                    .unwrap_or_else(|| panic!("Undefined variable: {}", name))
            }
            
            AstNode::BinaryOp { left, op, right } => {
                match op {
                    // Arithmetic operations
                    Operator::Add | Operator::Sub | Operator::Mul | Operator::Div => {
                        let left_val = self.eval(left);
                        let right_val = self.eval(right);
                        
                        // Convert to float if either operand is float
                        let (left_num, right_num) = match (&left_val, &right_val) {
                            (Value::Int(l), Value::Int(r)) => (*l as f64, *r as f64),
                            (Value::Float(l), Value::Int(r)) => (*l, *r as f64),
                            (Value::Int(l), Value::Float(r)) => (*l as f64, *r),
                            (Value::Float(l), Value::Float(r)) => (*l, *r),
                            _ => panic!("Expected numeric values in arithmetic operation"),
                        };
                        
                        let result = match op {
                            Operator::Add => left_num + right_num,
                            Operator::Sub => left_num - right_num,
                            Operator::Mul => left_num * right_num,
                            Operator::Div => {
                                if right_num == 0.0 {
                                    panic!("Division by zero");
                                }
                                left_num / right_num  // Always returns float
                            }
                            _ => unreachable!(),
                        };
                        
                        // Division always returns float, others return float if any operand was float
                        if matches!(op, Operator::Div) {
                            Value::Float(result)
                        } else {
                            match (&left_val, &right_val) {
                                (Value::Float(_), _) | (_, Value::Float(_)) => Value::Float(result),
                                _ => Value::Int(result as i64),
                            }
                        }
                    }
                    
                    // Comparison operations
                    Operator::Equal | Operator::NotEqual | 
                    Operator::LessThan | Operator::GreaterThan |
                    Operator::LessThanOrEqual | Operator::GreaterThanOrEqual => {
                        let left_val = self.eval(left);
                        let right_val = self.eval(right);
                        
                        let result = match (&left_val, &right_val) {
                            (Value::Int(l), Value::Int(r)) => {
                                match op {
                                    Operator::Equal => l == r,
                                    Operator::NotEqual => l != r,
                                    Operator::LessThan => l < r,
                                    Operator::GreaterThan => l > r,
                                    Operator::LessThanOrEqual => l <= r,
                                    Operator::GreaterThanOrEqual => l >= r,
                                    _ => unreachable!(),
                                }
                            }
                            (Value::Float(l), Value::Float(r)) => {
                                match op {
                                    Operator::Equal => l == r,
                                    Operator::NotEqual => l != r,
                                    Operator::LessThan => l < r,
                                    Operator::GreaterThan => l > r,
                                    Operator::LessThanOrEqual => l <= r,
                                    Operator::GreaterThanOrEqual => l >= r,
                                    _ => unreachable!(),
                                }
                            }
                            // Allow comparison between int and float
                            (Value::Int(l), Value::Float(r)) => {
                                let l = *l as f64;
                                match op {
                                    Operator::Equal => l == *r,
                                    Operator::NotEqual => l != *r,
                                    Operator::LessThan => l < *r,
                                    Operator::GreaterThan => l > *r,
                                    Operator::LessThanOrEqual => l <= *r,
                                    Operator::GreaterThanOrEqual => l >= *r,
                                    _ => unreachable!(),
                                }
                            }
                            (Value::Float(l), Value::Int(r)) => {
                                let r = *r as f64;
                                match op {
                                    Operator::Equal => *l == r,
                                    Operator::NotEqual => *l != r,
                                    Operator::LessThan => *l < r,
                                    Operator::GreaterThan => *l > r,
                                    Operator::LessThanOrEqual => *l <= r,
                                    Operator::GreaterThanOrEqual => *l >= r,
                                    _ => unreachable!(),
                                }
                            }
                            (Value::Bool(l), Value::Bool(r)) => {
                                match op {
                                    Operator::Equal => l == r,
                                    Operator::NotEqual => l != r,
                                    _ => panic!("Cannot use <, >, <=, >= on boolean values"),
                                }
                            }
                            (Value::String(l), Value::String(r)) => {
                                match op {
                                    Operator::Equal => l == r,
                                    Operator::NotEqual => l != r,
                                    _ => panic!("Cannot use <, >, <=, >= on string values"),
                                }
                            }
                            _ => panic!("Type mismatch in comparison"),
                        };
                        
                        Value::Bool(result)
                    }
                    
                    // Logical operations
                    Operator::And | Operator::Or | Operator::Xor | Operator::Nand => {
                        let left_val = self.eval(left);
                        let right_val = self.eval(right);
                        
                        let left_bool = self.to_bool(&left_val);
                        let right_bool = self.to_bool(&right_val);
                        
                        let result = match op {
                            Operator::And => left_bool && right_bool,
                            Operator::Or => left_bool || right_bool,
                            Operator::Xor => left_bool ^ right_bool,
                            Operator::Nand => !(left_bool && right_bool),
                            _ => unreachable!(),
                        };
                        
                        Value::Bool(result)
                    }
                }
            }
            
            AstNode::VarDecl { name, var_type } => {
                let default_val = match var_type {
                    VarType::Int => Value::Int(0),
                    VarType::Bool => Value::Bool(false),
                    VarType::Str => Value::String(String::new()),
                    VarType::Float => Value::Float(0.0),
                };
                self.variables.insert(name.clone(), default_val);
                self.variable_types.insert(name.clone(), var_type.clone());
                Value::None
            }
            
            AstNode::ConstDecl { name, var_type } => {
                let default_val = match var_type {
                    VarType::Int => Value::Int(0),
                    VarType::Bool => Value::Bool(false),
                    VarType::Str => Value::String(String::new()),
                    VarType::Float => Value::Float(0.0),
                };
                self.variables.insert(name.clone(), default_val);
                self.variable_types.insert(name.clone(), var_type.clone());
                self.constants.insert(name.clone());
                Value::None
            }
            
            AstNode::Assignment { name, value } => {
                // Check if trying to reassign a constant
                if self.constants.contains(name) {
                    panic!("Cannot reassign constant '{}'", name);
                }
                
                let val = self.eval(value);
                
                // Type checking: ensure the value matches the declared type
                if let Some(expected_type) = self.variable_types.get(name) {
                    let type_matches = match (expected_type, &val) {
                        (VarType::Int, Value::Int(_)) => true,
                        (VarType::Bool, Value::Bool(_)) => true,
                        (VarType::Str, Value::String(_)) => true,
                        (VarType::Float, Value::Float(_)) => true,
                        _ => false,
                    };
                    
                    if !type_matches {
                        let val_type = match &val {
                            Value::Int(_) => "int",
                            Value::Float(_) => "float",
                            Value::Bool(_) => "bool",
                            Value::String(_) => "str",
                            Value::None => "none",
                        };
                        let expected_type_str = match expected_type {
                            VarType::Int => "int",
                            VarType::Bool => "bool",
                            VarType::Str => "str",
                            VarType::Float => "float",
                        };
                        panic!(
                            "Type mismatch: cannot assign {} to variable '{}' of type {}",
                            val_type, name, expected_type_str
                        );
                    }
                }
                
                self.variables.insert(name.clone(), val.clone());
                val
            }
            
            AstNode::Printf(expr) => {
                let val = self.eval(expr);
                match val {
                    Value::Int(n) => println!("{}", n),
                    Value::Float(f) => println!("{}", f),
                    Value::Bool(b) => println!("{}", b),
                    Value::String(s) => println!("{}", s),
                    Value::None => println!("none"),
                }
                Value::None
            }
            
            AstNode::ToStr(expr) => {
                let val = self.eval(expr);
                let string = match val {
                    Value::Int(n) => n.to_string(),
                    Value::Float(f) => f.to_string(),
                    Value::Bool(b) => b.to_string(),
                    Value::String(s) => s, // Already a string
                    Value::None => "none".to_string(),
                };
                Value::String(string)
            }
            
            AstNode::ToInt(expr) => {
                let val = self.eval(expr);
                let int_val = match val {
                    Value::Int(n) => n,
                    Value::Float(f) => f.trunc() as i64,  // Truncate decimal places as per spec
                    Value::Bool(b) => if b { 1 } else { 0 },
                    Value::String(s) => {
                        // Try to parse the string as an integer
                        s.trim().parse::<i64>()
                            .unwrap_or_else(|_| panic!("Cannot convert '{}' to integer", s))
                    }
                    Value::None => 0,
                };
                Value::Int(int_val)
            }
            
            AstNode::Program(statements) => {
                let mut last_val = Value::None;
                for stmt in statements {
                    last_val = self.eval(stmt);
                }
                last_val
            }
        }
    }
    
    // Helper function to convert values to boolean
    // According to spec: true, false, 1, 0 can represent booleans
    fn to_bool(&self, value: &Value) -> bool {
        match value {
            Value::Bool(b) => *b,
            Value::Int(n) => *n != 0,  // 0 is false, any other number is true
            Value::Float(f) => *f != 0.0,  // 0.0 is false, any other float is true
            _ => panic!("Cannot convert {:?} to boolean", value),
        }
    }
}