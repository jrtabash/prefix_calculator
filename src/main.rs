pub mod pcalc_value;
pub mod pcalc_binary_ops;
pub mod pcalc_unary_ops;
pub mod pcalc_environment;
pub mod pcalc_code;
pub mod pcalc_lexer;
pub mod pcalc_parser;
pub mod pcalc_repl;

use pcalc_repl::REPL;

fn main() {
    let mut repl = REPL::new();
    repl.run();
}
