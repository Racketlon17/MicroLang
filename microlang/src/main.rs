use microlang::lexer::Lexer;
use microlang::parser::Parser;
use microlang::interpreter::Interpreter;
use std::fs;

fn main() {
    let filename = "test.mlang";
    let input = fs::read_to_string(filename)
        .expect("Failed to read file");
    
    let lexer = Lexer::new(&input);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    
    let mut interpreter = Interpreter::new();
    interpreter.eval(&ast);
}