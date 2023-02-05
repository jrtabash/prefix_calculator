use crate::pcalc_environment::Environment;
use crate::pcalc_keywords as keywords;
use crate::pcalc_parser::Parser;
use crate::pcalc_value::Value;
use std::io::{self, Write};

const CMD_ENV: &str = ":env";
const CMD_RESET: &str = ":reset";
const CMD_QUIT: &str = ":quit";
const CMD_HELP: &str = ":help";

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

    pub fn eval_expr(&mut self, expr: &str) {
        self.eval_and_print_line(expr);
    }

    pub fn run(&mut self) {
        let mut line = String::new();
        loop {
            if !self.prompt_and_read_line(&mut line) {
                continue;
            }

            let line_ref = line.trim();
            if line_ref == CMD_QUIT || line.is_empty() {
                println!();
                break;
            }

            if self.try_repl_command(line_ref) {
                continue;
            }

            self.eval_and_print_line(line_ref);
        }
    }

    pub fn display_startup_msg(&self) {
        println!("*****************************************************************");
        println!("*                       Prefix Calculator                       *");
        println!("*****************************************************************");
        self.print_help();
        println!("*****************************************************************");
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

    fn print_help(&self) {
        fn print_list(title: &str, kws: &keywords::NameList) {
            print!("{}: ", title);

            let mut count: u32 = 0;
            for sym in kws {
                count += 1;
                if count > 8 {
                    print!("\n               ");
                    count = 0;
                }
                print!("{} ", sym);
            }

            println!();
        }

        print_list("   Binary Ops", &keywords::binary_ops());
        print_list("    Unary Ops", &keywords::unary_ops());
        print_list("    Constants", &keywords::constants());
        print_list(" Special Vars", &vec![&self.last_var]);
        print_list("    REPL Cmds", &vec![CMD_ENV, CMD_RESET, CMD_QUIT, CMD_HELP]);
    }

    fn try_repl_command(&mut self, cmd: &str) -> bool {
        if cmd == CMD_ENV {
            self.env.show();
            return true;
        } else if cmd == CMD_RESET {
            self.reset_env();
            return true;
        } else if cmd == CMD_HELP {
            self.print_help();
            return true;
        }

        false
    }
}
