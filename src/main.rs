extern crate clap;

use clap::{App, Arg};
use prefix_calculator::pcalc_repl::REPL;

struct Arguments {
    force_int: bool,
    quiet: bool,
    batch: bool,
    expr: String
}

fn main() {
    let args = parse_args();
    run_repl(&args);
}

// --------------------------------------------------------------------------------

fn parse_args() -> Arguments {
    #[rustfmt::skip]
    let pargs = App::new("Prefix Calculator")
        .version("0.3.0")
        .about("Command line prefix calculator")
        .arg(Arg::with_name("force_int")
             .short("i")
             .long("int")
             .help("Force interactive mode. Use with -e/--expr option to force interactive mode"))
        .arg(Arg::with_name("quiet")
             .short("q")
             .long("quiet")
             .help("Disable startup message"))
        .arg(Arg::with_name("batch")
             .short("-b")
             .long("batch")
             .help("Enable batch mode"))
        .arg(Arg::with_name("expr")
             .short("e")
             .long("expr")
             .help("Evaluate expression. Use -i/--int to force interactive mode. \
                    Use semicolon ; to separate multiple expressions.")
             .takes_value(true))
        .get_matches();

    Arguments {
        force_int: pargs.is_present("force_int"),
        quiet: pargs.is_present("quiet"),
        batch: pargs.is_present("batch"),
        expr: match pargs.value_of("expr") {
            Some(e) => String::from(e),
            None => String::new()
        }
    }
}

fn run_repl(args: &Arguments) {
    let mut repl = REPL::new(args.batch);
    if !args.quiet {
        repl.display_startup_msg();
    }
    if !args.expr.is_empty() {
        repl.eval_expr(&args.expr);
    }
    if args.expr.is_empty() || args.force_int {
        repl.run();
    }
}
