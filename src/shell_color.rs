pub mod shell_color {
    use std::collections::HashMap;
    use crate::token_converter::token_converter;

    const RED: &str = "\x1b[31m";
    const GREEN: &str = "\x1b[32m";
    const YELLOW: &str = "\x1b[33m";
    const BLUE: &str = "\x1b[34m";
    const CYAN: &str = "\x1b[36m";
    const WHITE: &str = "\x1b[37m";
    const FILLED_BLACK: &str = "\x1b[40m";
    const FILLED_RED: &str = "\x1b[41m";
    const FILLED_GREEN: &str = "\x1b[42m";
    const FILLED_YELLOW: &str = "\x1b[43m";
    const FILLED_BLUE: &str = "\x1b[44m";
    const FILLED_MAGENTA: &str = "\x1b[45m";
    const FILLED_CYAN: &str = "\x1b[46m";
    const FILLED_WHITE: &str = "\x1b[47m";
    const RESET: &str = "\x1b[0m";
    
    pub fn get_converter() -> HashMap<String, Box<crate::token_converter::token_converter::Converter>> {
        vec![
            ("red",     token_converter::boxed_fn(|_| Some(RED.to_string()))),
            ("green",   token_converter::boxed_fn(|_| Some(GREEN.to_string()))),
            ("yellow",  token_converter::boxed_fn(|_| Some(YELLOW.to_string()))),
            ("blue",    token_converter::boxed_fn(|_| Some(BLUE.to_string()))),
            ("cyan",    token_converter::boxed_fn(|_| Some(CYAN.to_string()))),
            ("white",   token_converter::boxed_fn(|_| Some(WHITE.to_string()))),
            ("reset",   token_converter::boxed_fn(|_| Some(RESET.to_string()))),
            ("fred",    token_converter::boxed_fn(|_| Some(FILLED_RED.to_string()))),
            ("fgreen",  token_converter::boxed_fn(|_| Some(FILLED_GREEN.to_string()))),
            ("fyellow", token_converter::boxed_fn(|_| Some(FILLED_YELLOW.to_string()))),
            ("fblue",   token_converter::boxed_fn(|_| Some(FILLED_BLUE.to_string()))),
            ("fcyan",   token_converter::boxed_fn(|_| Some(FILLED_CYAN.to_string()))),
            ("fwhite",  token_converter::boxed_fn(|_| Some(FILLED_WHITE.to_string()))),
            ("fblack",  token_converter::boxed_fn(|_| Some(FILLED_BLACK.to_string()))),
        ].into_iter()
         .map(|(key, converter)| (key.to_string(), converter))
         .collect()
    }
}