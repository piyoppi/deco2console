mod shell_color;
use crate::shell_color::parser;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let source = &args[1];
    let colored = parser::convert(source);
    
    println!("{}", colored);

    parser::convert(source);
}