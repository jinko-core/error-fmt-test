use std::fmt::Display;

use colored::Colorize;

static mut LINE_COUNTER: usize = 0;

fn inc_line() {
    unsafe {
        LINE_COUNTER += 1;
    }
}

fn get_line() -> usize {
    unsafe { LINE_COUNTER }
}

fn show_line<S: Display, T: Display>(separator: S, line: T) {
    println!("{:5} {} {}", get_line(), separator, line);
    inc_line();
}

fn display_context(s: &str) {
    for line in s.lines() {
        show_line('|', line)
    }
}

fn display_code(s: &str) {
    for line in s.lines() {
        show_line("x".yellow(), line);
    }
}

fn main() {
    let context_start = r#"func upper_fn() {
    println("oh no");

    type Ctx;
    mut context = Ctx;
}

"#;

    let context_end = r#"
type After(contains: int);"#;

    let input = r#"func tc_error(arg: int) -> float {
    arg
}"#;

    println!("{}: {}", "error type".yellow(), "Typechecker".red());
    println!();

    display_context(context_start);
    display_code(input);
    display_context(context_end);

    println!();
    println!(
        "{}: invalid type returned in function `{}`: expected type `{}`, found type `{}`",
        "file.jk:7:5".yellow(),
        "tc_error".bold(),
        "float".purple(),
        "int".purple()
    );
}

