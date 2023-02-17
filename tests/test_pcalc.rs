use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process;

// Run a prefix_calculator command, using --quiet and --batch mode.
// Any printed output should be specified as a final xprint expression
// using add_expr function.
// Example:
//   PCalcCmd::new()
//     .add_expr("+ 1 2")
//     .add_expr("xprint last")
//     .expect_output("3")
//     .run()
struct PCalcCmd {
    expr: String,
    file: String,
    expected: String
}

impl PCalcCmd {
    pub fn new() -> Self {
        PCalcCmd {
            expr: String::new(),
            file: String::new(),
            expected: String::new()
        }
    }

    pub fn add_expr(&mut self, expr: &str) -> &mut Self {
        if !self.expr.is_empty() {
            self.expr.push_str(";");
        }
        self.expr.push_str(expr);
        self
    }

    pub fn with_file(&mut self, filename: &str, content: &str) -> &mut Self {
        self.file = format!("/tmp/{}", filename);
        self.make_file(content);
        self
    }

    pub fn expect_output(&mut self, expected: &str) -> &mut Self {
        self.expected = expected.to_string();
        self
    }

    pub fn run(&mut self) {
        assert!(!self.expr.is_empty() || !self.file.is_empty());

        let mut cmd = self.make_command();
        match cmd.output() {
            Ok(out) => {
                let actual = out.stdout.escape_ascii().to_string();
                let expect = format!("{}{}", self.expected, if self.expected.is_empty() { "" } else { "\\n" });
                assert_eq!(actual, expect);
            }
            Err(e) => {
                eprintln!("Failed to run command - {}", e);
            }
        }
    }

    fn make_command(&self) -> process::Command {
        let mut cmd = process::Command::new("target/debug/prefix_calculator");
        cmd.arg("-q");
        cmd.arg("-b");
        if !self.expr.is_empty() {
            cmd.arg("-e").arg(self.expr.as_str());
        }
        if !self.file.is_empty() {
            cmd.arg("-f").arg(self.file.as_str());
        }
        cmd
    }

    fn make_file(&self, content: &str) {
        match fs::File::create(&self.file) {
            Ok(mut file) => {
                for line in content.trim().split('\n') {
                    if let Err(err) = writeln!(file, "{}", line) {
                        println!("make_file - failed to write file - {}", err);
                        break;
                    }
                }
            }
            Err(err) => eprintln!("make_file - failed to create file - {}", err)
        };
    }
}

impl Drop for PCalcCmd {
    fn drop(&mut self) {
        if !self.file.is_empty() {
            let path = PathBuf::from(self.file.as_str());
            if path.exists() {
                if fs::remove_file(path.as_path()).is_err() {
                    eprintln!("Drop - failed to remove {}", self.file);
                }
            }
        }
    }
}

// --------------------------------------------------------------------------------
// TESTS

#[test]
fn test_pcalc_no_output() {
    PCalcCmd::new().add_expr("+ 1 2").run();
}

#[test]
fn test_pcalc_initial_last() {
    PCalcCmd::new().add_expr("xprint last").expect_output("0").run();
}

#[test]
fn test_pcalc_basic() {
    PCalcCmd::new().add_expr("+ 1 2").add_expr("xprint last").expect_output("3").run();
}

#[test]
fn test_pcalc_vars() {
    PCalcCmd::new()
        .add_expr("var x 3")
        .add_expr("var y 4")
        .add_expr("var z sqrt + ^ x 2 ^ y 2")
        .add_expr("xprint z")
        .expect_output("5")
        .run();

    PCalcCmd::new()
        .add_expr("var x 5")
        .add_expr("= x * 2 x")
        .add_expr("xprint x")
        .expect_output("10")
        .run();
}

#[test]
fn test_pcalc_file_no_output() {
    PCalcCmd::new().with_file("test_file_no_output.pcalc", "+ 1 2").run();
}

#[test]
fn test_pcalc_file() {
    PCalcCmd::new()
        .add_expr("xprint z")
        .with_file(
            "test_pcalc_file.pcalc",
            "var x 3\n\
             var y 4\n\
             var z sqrt + ^ x 2 ^ y 2"
        )
        .expect_output("5")
        .run();
}
