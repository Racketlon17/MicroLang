#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Number(i64),
    FloatLiteral(f64),
    Plus,
    Minus,
    Star,
    Slash,
    // Keywords
    Var,
    Const,
    Int,
    Bool,
    Str,
    Float,
    True,
    False,
    Printf,
    ToStr,
    ToInt,
    // Logical operators
    And,
    Or,
    Xor,
    Nand,
    // Comparison operators
    EqualEqual,
    NotEqual,
    LessThanOrEqual,
    GreaterThanOrEqual,
    LessThan,
    GreaterThan,
    // Symbols
    Identifier(String),
    StringLiteral(String),
    DoubleColon,
    Equals,
    LParen,
    RParen,
    Newline,
    Semicolon,
    
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
}

impl Token {
    pub fn new(token_type: TokenType) -> Self {
        Token { token_type }
    }
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }
    
    fn current_char(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }
    
    fn advance(&mut self) {
        self.position += 1;
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch == ' ' || ch == '\t' || ch == '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    fn read_number(&mut self) -> Token {
        let mut num_str = String::new();
        let mut is_float = false;
        
        while let Some(ch) = self.current_char() {
            if ch.is_ascii_digit() {
                num_str.push(ch);
                self.advance();
            } else if ch == '.' && !is_float {
                // Check if next char is a digit (to avoid catching '..' or other operators)
                if let Some(next_ch) = self.input.get(self.position + 1) {
                    if next_ch.is_ascii_digit() {
                        is_float = true;
                        num_str.push(ch);
                        self.advance();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        if is_float {
            let float_val = num_str.parse::<f64>().expect("Failed to parse float");
            Token::new(TokenType::FloatLiteral(float_val))
        } else {
            let int_val = num_str.parse::<i64>().expect("Failed to parse number");
            Token::new(TokenType::Number(int_val))
        }
    }
    
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        
        match self.current_char() {
            None => Token::new(TokenType::Eof),
            
            Some('\n') => {
                self.advance();
                Token::new(TokenType::Newline)
            }
            
            Some(';') => {
                self.advance();
                Token::new(TokenType::Semicolon)
            }
            
            Some(ch) if ch.is_ascii_digit() => {
                self.read_number()
            }
            
            Some(ch) if ch.is_alphabetic() || ch == '_' => {
                let ident = self.read_identifier();
                let token_type = match ident.as_str() {
                    "var" => TokenType::Var,
                    "const" => TokenType::Const,
                    "int" => TokenType::Int,
                    "bool" => TokenType::Bool,
                    "str" => TokenType::Str,
                    "float" => TokenType::Float,
                    "true" => TokenType::True,
                    "false" => TokenType::False,
                    "printf" => TokenType::Printf,
                    "toStr" => TokenType::ToStr,
                    "toInt" => TokenType::ToInt,
                    "and" => TokenType::And,
                    "or" => TokenType::Or,
                    "xor" => TokenType::Xor,
                    "nand" => TokenType::Nand,
                    _ => TokenType::Identifier(ident),
                };
                Token::new(token_type)
            }
            
            Some('(') => {
                self.advance();
                Token::new(TokenType::LParen)
            }

            Some(')') => {
                self.advance();
                Token::new(TokenType::RParen)
            }

            Some('"') => {
                let string = self.read_string();
                Token::new(TokenType::StringLiteral(string))
            }
            
            Some('+') => {
                self.advance();
                Token::new(TokenType::Plus)
            }
            
            Some('-') => {
                self.advance();
                Token::new(TokenType::Minus)
            }
            
            Some('*') => {
                self.advance();
                Token::new(TokenType::Star)
            }
            
            Some('/') => {
                self.advance();
                Token::new(TokenType::Slash)
            }
            
            Some('=') => {
                self.advance();
                if self.current_char() == Some('=') {
                    self.advance();
                    Token::new(TokenType::EqualEqual)
                } else {
                    Token::new(TokenType::Equals)
                }
            }
            
            Some('!') => {
                self.advance();
                if self.current_char() == Some('=') {
                    self.advance();
                    Token::new(TokenType::NotEqual)
                } else {
                    panic!("Expected '!=' but got '!'");
                }
            }
            
            Some('<') => {
                self.advance();
                if self.current_char() == Some('=') {
                    self.advance();
                    Token::new(TokenType::LessThanOrEqual)
                } else {
                    Token::new(TokenType::LessThan)
                }
            }
            
            Some('>') => {
                self.advance();
                if self.current_char() == Some('=') {
                    self.advance();
                    Token::new(TokenType::GreaterThanOrEqual)
                } else {
                    Token::new(TokenType::GreaterThan)
                }
            }
            
            Some(':') => {
                self.advance();
                if self.current_char() == Some(':') {
                    self.advance();
                    Token::new(TokenType::DoubleColon)
                } else {
                    panic!("Expected '::', got ':'");
                }
            }
            
            Some(ch) => {
                panic!("Unexpected character: '{}'", ch);
            }
        }
    }
    
    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        
        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        ident
    }

    fn read_string(&mut self) -> String {
        self.advance();
        let mut string = String::new();

        while let Some(ch) = self.current_char() {
            if ch == '"' {
                self.advance();
                return string;
            }
            string.push(ch);
            self.advance();
        }
        panic!("Unterminated string")
    }
}