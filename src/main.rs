pub mod pcalc_value;
pub mod pcalc_binary_ops;
pub mod pcalc_unary_ops;
pub mod pcalc_environment;
pub mod pcalc_code;
pub mod pcalc_lexer;
pub mod pcalc_parser;

use pcalc_environment::Environment;
use pcalc_parser::Parser;

fn main() {
    let mut env = Environment::new();
    let mut parser = Parser::new();

    parser.parse("var x 3").unwrap().eval(&mut env).unwrap();
    parser.parse("var y 4").unwrap().eval(&mut env).unwrap();
    parser.parse("var z sqrt + ^ x 2 ^ y 2").unwrap().eval(&mut env).unwrap();
    println!("x = {}", env.get("x").unwrap());
    println!("y = {}", env.get("y").unwrap());
    println!("z = {}", env.get("z").unwrap());
}
