mod shell_color;
use crate::shell_color::parser;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    parser::convert(&args[1]);
}