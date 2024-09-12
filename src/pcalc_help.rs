use crate::pcalc_keywords as keywords;

pub fn print_help(special_vars: &Vec<&str>, repl_commands: &Vec<&str>) {
    print_list("   Binary Ops", &keywords::binary_ops());
    print_list("    Unary Ops", &keywords::unary_ops());
    print_list("    Vars Mgmt", &vec![keywords::DEFVAR, keywords::SETVAR]);
    print_list("    Ftns Mgmt", &vec![keywords::DEFUN, keywords::FUNCALL]);
    print_list("    Ctrl Flow", &vec![keywords::IF]);
    print_list("    Constants", &keywords::constants());
    print_list(" Special Ftns", &keywords::special_ftns());
    print_list(" Special Vars", special_vars);
    print_list("    REPL Cmds", repl_commands);
}

pub fn print_examples() {
    print_example(
        1,
        "Basic",
        "> var x 5\n\
         5\n\
         > * 2 + x 20\n\
         50\n\
         > sqrt + ^ 3 2 ^ 4 2\n\
         5\n\
         > / * 3.5 pi 2\n\
         5.497787143782138\n\
         > max x last\n\
         5.497787143782138\n\
         > and asbool 5 true\n\
         true\n\
         > + 5 asnum true\n\
         6"
    );
    print_example(
        2,
        "Functions",
        "> def dist x1 y1 x2 y2\n\
         >>> begin\n\
         >>> var dx2 ^ - x2 x1 2\n\
         >>> var dy2 ^ - y2 y1 2\n\
         >>> sqrt + dx2 dy2\n\
         >>> end\n\
         true\n\
         >\n\
         > call dist 3 4 6 8 cend\n\
         5\n\
         >\n\
         > def near x1 y1 x2 y2 begin < call dist x1 y1 x2 y2 cend 1.0 end\n\
         true\n\
         >\n\
         > call near 3 4 3.5 4.5 cend\n\
         true"
    );
    print_example(
        3,
        "Conditionals",
        "> var x 5\n\
         5\n\
         > var y 10\n\
         10\n\
         > if <= x 5 ? = x + x 1 : = y + y 1 fi\n\
         6\n\
         > if > x 10 ? = x + x 1 : = y + y 1 fi\n\
         11\n\
         > if < x 10 ? x fi\n\
         6\n\
         > if > x 10 ? x fi\n\
         false"
    );
}

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

fn print_example(cnt: i32, name: &str, text: &str) {
    println!("\n----------");
    println!("Example {} - {}", cnt, name);
    println!("{}", text);
}
