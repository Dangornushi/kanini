pub mod calc;

use crate::calc::expr_eval;

use std::io::Write;

fn main() {
    unsafe { backtrace_on_stack_overflow::enable() };
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();
        match expr_eval(&s) {
            Ok(val) => println!("OK: {:?}", val),
            _ => println!("構文に誤りがあります"),
        }
    }
}
// Ok(val) => println!("{}", val),
