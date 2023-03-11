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

#[test]
fn test_pcalc_file_multi_expr_line() {
    PCalcCmd::new()
        .add_expr("xprint C")
        .with_file(
            "test_pcalc_file_multi_expr_line",
            "var F 50; var C - F 32\n\
             = C / * C 5 9"
        )
        .expect_output("10")
        .run();
}

#[test]
fn test_pcalc_empty_file() {
    PCalcCmd::new()
        .add_expr("xprint last")
        .with_file("test_pcalc_empty_file", "")
        .expect_output("0")
        .run();
}

#[test]
fn test_pcalc_expr_defun() {
    PCalcCmd::new()
        .add_expr("def add3 x y z begin + x + y z end")
        .add_expr("xprint call add3 1 2 3 cend")
        .expect_output("6")
        .run();
}

#[test]
fn test_pcalc_file_defun() {
    PCalcCmd::new()
        .add_expr("xprint call add3 2 3 4 cend")
        .with_file(
            "test_pcalc_file_defun",
            "def add3 x y z\n\
             begin\n\
             var temp + x y\n\
             + temp z\n\
             end\n"
        )
        .expect_output("9")
        .run();
}

#[test]
fn test_pcalc_funcalls() {
    PCalcCmd::new()
        .add_expr("xprint call add4 2 3 4 6 cend")
        .with_file(
            "test_pcalc_funcalls",
            "def add x y begin + x y end\n\
             \n\
             def add4 a b c d\n\
             begin\n\
             + call add a b cend call add c d cend\n\
             end\n"
        )
        .expect_output("15")
        .run();
}

#[test]
fn test_pcalc_funcalls2() {
    PCalcCmd::new()
        .add_expr("xprint call near 3 4 4 5 cend")
        .add_expr("xprint call near 3 4 3.5 4.5 cend")
        .with_file(
            "test_pcalc_funcalls2",
            "def dist x1 y1 x2 y2\n\
             begin\n\
             var dx2 ^ - x2 x1 2\n\
             var dy2 ^ - y2 y1 2\n\
             sqrt + dx2 dy2\n\
             end\n\
             \n\
             def near x1 y1 x2 y2\n\
             begin\n\
             <= call dist x1 y1 x2 y2 cend 1.0\n\
             end\n"
        )
        .expect_output("false\\ntrue")
        .run();
}
