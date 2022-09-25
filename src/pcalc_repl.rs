use crate::pcalc_environment::Environment;
use crate::pcalc_parser::Parser;
use crate::pcalc_value::Value;
use std::io::{self, Write};

pub struct REPL {
    prompt: String,
    last_var: String,
    env: Environment,
    parser: Parser
}

impl REPL {
    pub fn new() -> Self {
        let mut repl = REPL {
            prompt: String::from("> "),
            last_var: String::from("last"),
            env: Default::default(),
            parser: Default::default()
        };
        repl.reset_env();
        repl
    }

    pub fn with_expr(expr: &str) -> Self {
        let mut repl = REPL::new();
        repl.eval_and_print_line(expr);
        repl
    }

    pub fn run(&mut self) {
        let mut line = String::new();
        loop {
            if !self.prompt_and_read_line(&mut line) {
                continue;
            }

            let line_ref = line.trim();
            if line_ref == ":quit" || line.is_empty() {
                println!();
                break;
            }

            if self.try_repl_command(line_ref) {
                continue;
            }

            self.eval_and_print_line(line_ref);
        }
    }

    // --------------------------------------------------------------------------------
    // Private Functions

    fn prompt_and_read_line(&self, line: &mut String) -> bool {
        line.clear();

        print!("{}", self.prompt);
        match io::stdout().flush() {
            Ok(()) => {}
            Err(err) => {
                eprintln!("WriteError: {}", err);
                return false;
            }
        }

        match io::stdin().read_line(line) {
            Ok(_size) => true,
            Err(err) => {
                eprintln!("ReadError: {}", err);
                false
            }
        }
    }

    fn eval_and_print_line(&mut self, line: &str) {
        for sub_expr in line.split(';').map(|e| e.trim()) {
            if sub_expr.is_empty() {
                continue;
            }

            if !self.eval_and_print(sub_expr) {
                break;
            }
        }
    }

    fn eval_and_print(&mut self, expr: &str) -> bool {
        match self.parser.parse(expr) {
            Ok(code) => match code.eval(&mut self.env) {
                Ok(value) => {
                    println!("{}", value);
                    self.env.set(&self.last_var, value).unwrap();
                    true
                }
                Err(err) => {
                    eprintln!("EvalError: {}", err);
                    false
                }
            },
            Err(err) => {
                eprintln!("ParseError: {}", err);
                false
            }
        }
    }

    fn reset_env(&mut self) {
        self.env.reset();
        self.env.def(&self.last_var, Value::from_num(0.0)).unwrap();
    }

    fn try_repl_command(&mut self, cmd: &str) -> bool {
        if cmd == ":env" {
            self.env.show();
            return true;
        } else if cmd == ":reset" {
            self.reset_env();
            return true;
        }

        false
    }
}

impl Default for REPL {
    fn default() -> Self {
        Self::new()
    }
}
