mod shell_color;
mod converter;
mod tokenizer;
mod token_converter;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let source = &args[1];
    let colored = crate::converter::converter::convert(
        source,
        crate::shell_color::shell_color::get_converter()
    );
    
    println!("{}", colored);
}