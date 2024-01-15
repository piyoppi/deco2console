mod shell_color;
mod converter;
mod tokenizer;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let source = &args[1];
    let colored = parser::convert(source);
    
    println!("{}", colored);

    parser::convert(source);
}