use crate::pcalc_environment::Environment;
use crate::pcalc_help as help;
use crate::pcalc_parser::Parser;
use crate::pcalc_value::Value;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, Write};

const CMD_ENV: &str = ":env";
const CMD_RESET: &str = ":reset";
const CMD_QUIT: &str = ":quit";
const CMD_BATCH: &str = ":batch";
const CMD_LAST: &str = ":last";
const CMD_HELP: &str = ":help";
const CMD_EXAMPLES: &str = ":examples";

pub struct REPL {
    prompt: String,
    alt_prompt: String,
    last_var: String,
    env: Environment,
    parser: Parser,
    batch: bool
}

impl REPL {
    pub fn new(batch: bool) -> Self {
        let mut repl = REPL {
            prompt: String::from("> "),
            alt_prompt: String::from(">>> "),
            last_var: String::from("last"),
            env: Default::default(),
            parser: Default::default(),
            batch
        };
        repl.reset_env();
        repl
    }

    #[inline(always)]
    pub fn eval_expr(&mut self, expr: &str) {
        self.eval_and_print_line(expr);
    }

    pub fn load_file(&mut self, filename: &str) {
        match File::open(filename) {
            Ok(file) => {
                let mut reader = BufReader::new(file);
                let mut content = String::new();
                match reader.read_to_string(&mut content) {
                    Ok(_) => {
                        self.eval_and_print_multi_line(&content);
                    }
                    Err(e) => eprintln!("Load file error: {}", e)
                }
            }
            Err(e) => eprintln!("Load file error: {}", e)
        };
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
        self.print_batch();
        println!("*****************************************************************");
    }

    // --------------------------------------------------------------------------------
    // Private Functions

    fn prompt_and_read_line(&self, line: &mut String) -> bool {
        line.clear();

        print!("{}", if self.parser.is_empty() { &self.prompt } else { &self.alt_prompt });
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

    fn eval_and_print_line(&mut self, line: &str) -> bool {
        for sub_expr in line.split(';').map(|e| e.trim()) {
            if sub_expr.is_empty() {
                continue;
            }
            if !self.eval_and_print(sub_expr) {
                return false;
            }
        }
        true
    }

    fn eval_and_print_multi_line(&mut self, exprs: &str) -> bool {
        for line in exprs.lines() {
            if line.is_empty() {
                continue;
            }
            if !self.eval_and_print_line(line) {
                return false;
            }
        }
        true
    }

    fn eval_and_print(&mut self, expr: &str) -> bool {
        match self.parser.parse(expr) {
            Ok(code) => {
                if !code.is_evaluable() {
                    return true;
                }

                match code.eval(&mut self.env) {
                    Ok(value) => {
                        if !self.batch {
                            println!("{}", value);
                        }
                        self.env.set_var(&self.last_var, value).unwrap();
                        true
                    }
                    Err(err) => {
                        eprintln!("EvalError: {}", err);
                        false
                    }
                }
            }
            Err(err) => {
                eprintln!("ParseError: {}", err);
                false
            }
        }
    }

    fn reset_env(&mut self) {
        self.env.reset();
        self.env.def_var(&self.last_var, Value::from_num(0.0)).unwrap();
    }

    fn toggle_batch(&mut self) {
        self.batch = !self.batch;
        self.print_batch();
    }

    fn print_batch(&self) {
        println!("batch mode {}", if self.batch { "on" } else { "off" });
    }

    fn print_last(&self) {
        match self.env.get_var(&self.last_var) {
            Ok(val) => println!("{}", val),
            Err(err) => eprintln!("ParseError: {}", err)
        };
    }

    fn print_help(&self) {
        help::print_help(
            &vec![&self.last_var],
            &vec![CMD_ENV, CMD_RESET, CMD_QUIT, CMD_BATCH, CMD_LAST, CMD_HELP, CMD_EXAMPLES]
        );
    }

    fn try_repl_command(&mut self, cmd: &str) -> bool {
        if cmd == CMD_ENV {
            self.env.show();
            return true;
        } else if cmd == CMD_RESET {
            self.reset_env();
            return true;
        } else if cmd == CMD_BATCH {
            self.toggle_batch();
            return true;
        } else if cmd == CMD_LAST {
            self.print_last();
            return true;
        } else if cmd == CMD_HELP {
            self.print_help();
            return true;
        } else if cmd == CMD_EXAMPLES {
            help::print_examples();
            return true;
        }

        false
    }
}
