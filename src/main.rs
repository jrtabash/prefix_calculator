extern crate clap;

use clap::{App, Arg};
use prefix_calculator::pcalc_repl::REPL;

struct Arguments {
    force_int: bool,
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
        .arg(Arg::with_name("expr")
             .short("e")
             .long("expr")
             .help("Evaluate expression. Use -i/--int to force interactive mode. \
                    Use semicolon ; to separate multiple expressions.")
             .takes_value(true))
        .get_matches();

    Arguments {
        force_int: pargs.is_present("force_int"),
        expr: match pargs.value_of("expr") {
            Some(e) => String::from(e),
            None => String::new()
        }
    }
}

fn run_repl(args: &Arguments) {
    let mut repl = if args.expr.is_empty() {
        Default::default()
    } else {
        REPL::with_expr(&args.expr)
    };
    if args.expr.is_empty() || args.force_int {
        repl.run();
    }
}
