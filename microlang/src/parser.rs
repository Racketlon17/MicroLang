use crate::lexer::{Lexer, Token, TokenType};
use crate::ast::{AstNode, Operator, VarType};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
        }
    }
    
    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }
    
    fn skip_newlines(&mut self) {
        while self.current_token.token_type == TokenType::Newline 
           || self.current_token.token_type == TokenType::Semicolon {
            self.advance();
        }
    }
    
    pub fn parse(&mut self) -> AstNode {
        let mut statements = Vec::new();
        
        self.skip_newlines();
        
        while self.current_token.token_type != TokenType::Eof {
            statements.push(self.parse_statement());
            self.skip_newlines();
        }
        
        AstNode::Program(statements)
    }
    
    fn parse_statement(&mut self) -> AstNode {
        match &self.current_token.token_type {
            TokenType::Var => self.parse_var_decl(),
            TokenType::Const => self.parse_const_decl(),
            TokenType::Printf => self.parse_printf(),
            TokenType::ToStr => self.parse_tostr(),
            TokenType::ToInt => self.parse_toint(),
            TokenType::Identifier(name) => {
                let name = name.clone();
                self.advance();
                
                if self.current_token.token_type == TokenType::Equals {
                    self.advance();
                    let value = self.parse_expression();
                    AstNode::Assignment {
                        name,
                        value: Box::new(value),
                    }
                } else {
                    // It's just an expression starting with an identifier
                    // We need to backtrack and parse the full expression
                    // For now, build from the identifier we already consumed
                    let mut left = AstNode::Identifier(name);
                    
                    // Continue parsing the rest of the expression
                    // This handles cases like: x + 2 or x and y
                    left = self.continue_expression(left);
                    
                    left
                }
            }
            _ => self.parse_expression(),
        }
    }
    
    // Helper to continue parsing an expression when we've already consumed the first term
    fn continue_expression(&mut self, left: AstNode) -> AstNode {
        let mut current = left;
        
        // Arithmetic operations (higher precedence)
        while matches!(self.current_token.token_type, TokenType::Plus | TokenType::Minus) {
            let op = match self.current_token.token_type {
                TokenType::Plus => Operator::Add,
                TokenType::Minus => Operator::Sub,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_term();
            current = AstNode::BinaryOp {
                left: Box::new(current),
                op,
                right: Box::new(right),
            };
        }
        
        // Logical operations (lower precedence)
        while matches!(self.current_token.token_type, 
                     TokenType::And | TokenType::Or | TokenType::Xor | TokenType::Nand) {
            let op = match self.current_token.token_type {
                TokenType::And => Operator::And,
                TokenType::Or => Operator::Or,
                TokenType::Xor => Operator::Xor,
                TokenType::Nand => Operator::Nand,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_arithmetic_expression();
            current = AstNode::BinaryOp {
                left: Box::new(current),
                op,
                right: Box::new(right),
            };
        }
        
        current
    }
    
    fn parse_var_decl(&mut self) -> AstNode {
        self.advance(); // consume 'var'
        
        let name = if let TokenType::Identifier(n) = &self.current_token.token_type {
            n.clone()
        } else {
            panic!("Expected identifier after 'var'");
        };
        self.advance();
        
        if self.current_token.token_type != TokenType::DoubleColon {
            panic!("Expected '::' after variable name");
        }
        self.advance();
        
        let var_type = match self.current_token.token_type {
            TokenType::Int => VarType::Int,
            TokenType::Bool => VarType::Bool,
            TokenType::Str => VarType::Str,
            TokenType::Float => VarType::Float,
            _ => panic!("Expected type after '::'"),
        };
        self.advance();
        
        AstNode::VarDecl { name, var_type }
    }
    
    fn parse_const_decl(&mut self) -> AstNode {
        self.advance(); // consume 'const'
        
        let name = if let TokenType::Identifier(n) = &self.current_token.token_type {
            n.clone()
        } else {
            panic!("Expected identifier after 'const'");
        };
        self.advance();
        
        if self.current_token.token_type != TokenType::DoubleColon {
            panic!("Expected '::' after constant name");
        }
        self.advance();
        
        let var_type = match self.current_token.token_type {
            TokenType::Int => VarType::Int,
            TokenType::Bool => VarType::Bool,
            TokenType::Str => VarType::Str,
            TokenType::Float => VarType::Float,
            _ => panic!("Expected type after '::'"),
        };
        self.advance();
        
        AstNode::ConstDecl { name, var_type }
    }

    fn parse_printf(&mut self) -> AstNode {
        self.advance(); // consume 'printf'
        
        if self.current_token.token_type != TokenType::LParen {
            panic!("Expected '(' after 'printf'");
        }
        self.advance();
        
        let expr = self.parse_expression();
        
        if self.current_token.token_type != TokenType::RParen {
            panic!("Expected ')' after printf argument");
        }
        self.advance();
        
        AstNode::Printf(Box::new(expr))
    }
    
    fn parse_tostr(&mut self) -> AstNode {
        self.advance(); // consume 'toStr'
        
        if self.current_token.token_type != TokenType::LParen {
            panic!("Expected '(' after 'toStr'");
        }
        self.advance();
        
        let expr = self.parse_expression();
        
        if self.current_token.token_type != TokenType::RParen {
            panic!("Expected ')' after toStr argument");
        }
        self.advance();
        
        AstNode::ToStr(Box::new(expr))
    }
    
    fn parse_toint(&mut self) -> AstNode {
        self.advance(); // consume 'toInt'
        
        if self.current_token.token_type != TokenType::LParen {
            panic!("Expected '(' after 'toInt'");
        }
        self.advance();
        
        let expr = self.parse_expression();
        
        if self.current_token.token_type != TokenType::RParen {
            panic!("Expected ')' after toInt argument");
        }
        self.advance();
        
        AstNode::ToInt(Box::new(expr))
    }    
    
    // PRECEDENCE HIERARCHY (lowest to highest):
    // 1. Logical OR
    // 2. Logical XOR  
    // 3. Logical AND/NAND
    // 4. Comparison (==, !=, etc.) <- Future addition
    // 5. Arithmetic (+, -)
    // 6. Term (*, /)
    // 7. Factor (numbers, bools, identifiers, parens)
    
    fn parse_expression(&mut self) -> AstNode {
        self.parse_logical_or()
    }
    
    fn parse_logical_or(&mut self) -> AstNode {
        let mut left = self.parse_logical_xor();
        
        while matches!(self.current_token.token_type, TokenType::Or) {
            self.advance();
            let right = self.parse_logical_xor();
            left = AstNode::BinaryOp {
                left: Box::new(left),
                op: Operator::Or,
                right: Box::new(right),
            };
        }
        
        left
    }
    
    fn parse_logical_xor(&mut self) -> AstNode {
        let mut left = self.parse_logical_and();
        
        while matches!(self.current_token.token_type, TokenType::Xor) {
            self.advance();
            let right = self.parse_logical_and();
            left = AstNode::BinaryOp {
                left: Box::new(left),
                op: Operator::Xor,
                right: Box::new(right),
            };
        }
        
        left
    }
    
    fn parse_logical_and(&mut self) -> AstNode {
        let mut left = self.parse_comparison();
        
        while matches!(self.current_token.token_type, TokenType::And | TokenType::Nand) {
            let op = match self.current_token.token_type {
                TokenType::And => Operator::And,
                TokenType::Nand => Operator::Nand,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_comparison();
            left = AstNode::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        
        left
    }
    
    fn parse_comparison(&mut self) -> AstNode {
        let mut left = self.parse_arithmetic_expression();
        
        while matches!(self.current_token.token_type, 
                     TokenType::EqualEqual | TokenType::NotEqual | 
                     TokenType::LessThan | TokenType::GreaterThan |
                     TokenType::LessThanOrEqual | TokenType::GreaterThanOrEqual) {
            let op = match self.current_token.token_type {
                TokenType::EqualEqual => Operator::Equal,
                TokenType::NotEqual => Operator::NotEqual,
                TokenType::LessThan => Operator::LessThan,
                TokenType::GreaterThan => Operator::GreaterThan,
                TokenType::LessThanOrEqual => Operator::LessThanOrEqual,
                TokenType::GreaterThanOrEqual => Operator::GreaterThanOrEqual,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_arithmetic_expression();
            left = AstNode::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        
        left
    }
    
    fn parse_arithmetic_expression(&mut self) -> AstNode {
        let mut left = self.parse_term();
        
        while matches!(self.current_token.token_type, TokenType::Plus | TokenType::Minus) {
            let op = match self.current_token.token_type {
                TokenType::Plus => Operator::Add,
                TokenType::Minus => Operator::Sub,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_term();
            left = AstNode::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        
        left
    }
    
    fn parse_term(&mut self) -> AstNode {
        let mut left = self.parse_factor();
        
        while matches!(self.current_token.token_type, TokenType::Star | TokenType::Slash) {
            let op = match self.current_token.token_type {
                TokenType::Star => Operator::Mul,
                TokenType::Slash => Operator::Div,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_factor();
            left = AstNode::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        
        left
    }
    
    fn parse_factor(&mut self) -> AstNode {
        match self.current_token.token_type.clone() {
            TokenType::Number(n) => {
                self.advance();
                AstNode::Number(n)
            }
            TokenType::FloatLiteral(f) => {
                self.advance();
                AstNode::Float(f)
            }
            TokenType::True => {
                self.advance();
                AstNode::Boolean(true)
            }
            TokenType::False => {
                self.advance();
                AstNode::Boolean(false)
            }
            TokenType::StringLiteral(s) => {
                self.advance();
                AstNode::StringLiteral(s)
            }
            TokenType::Identifier(name) => {
                self.advance();
                AstNode::Identifier(name)
            }
            TokenType::LParen => {
                self.advance();
                let expr = self.parse_expression();
                if self.current_token.token_type != TokenType::RParen {
                    panic!("Expected ')' after expression");
                }
                self.advance();
                expr
            }
            TokenType::ToStr => self.parse_tostr(),
            TokenType::ToInt => self.parse_toint(),
            _ => panic!("Expected number, float, boolean, string, identifier, function call, or '(', got {:?}", self.current_token.token_type),
        }
    }
}