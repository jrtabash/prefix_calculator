use std::io::{self, Write};
use crate::pcalc_value::Value;
use crate::pcalc_environment::Environment;
use crate::pcalc_parser::Parser;

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
            env: Environment::new(),
            parser: Parser::new()
        };
        repl.env.def(&repl.last_var, Value::from_num(0.0)).unwrap();
        repl
    }

    pub fn run(&mut self) {
        let mut line = String::new();
        loop {
            if !self.prompt_and_read(&mut line) {
                continue;
            }

            let line_ref = line.trim();
            if line_ref == ":quit" || line == "" {
                println!();
                break;
            }

            if line_ref == "" {
                continue;
            }

            self.eval_and_print(line_ref);
        }
    }

    // --------------------------------------------------------------------------------
    // Private Functions

    fn prompt_and_read(&self, line: &mut String) -> bool {
        line.clear();

        print!("{}", self.prompt);
        match io::stdout().flush() {
            Ok(()) => {},
            Err(err) => {
                eprintln!("WriteError: {}", err);
                return false;
            }
        }

        match io::stdin().read_line(line) {
            Ok(_size) => { true },
            Err(err) => {
                eprintln!("ReadError: {}", err);
                false
            }
        }
    }

    fn eval_and_print(&mut self, line: &str) {
        match self.parser.parse(line) {
            Ok(code) => {
                match code.eval(&mut self.env) {
                    Ok(value) => {
                        println!("{}", value);
                        self.env.set(&self.last_var, value).unwrap();
                    },
                    Err(err) => {
                        eprintln!("EvalError: {}", err);
                    }
                }
            },
            Err(err) => {
                eprintln!("ParseError: {}", err);
            }
        }
    }
}
