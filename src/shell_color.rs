pub mod shell_color {
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
  
    pub fn convert_color(token: &str) -> Option<String> {
        match &token[..] {
            "red" => Some(RED.to_string()),
            "green" => Some(GREEN.to_string()),
            "yellow" => Some(YELLOW.to_string()),
            "blue" => Some(BLUE.to_string()),
            "cyan" => Some(CYAN.to_string()),
            "white" => Some(WHITE.to_string()),
            "reset" => Some(RESET.to_string()),
            "fred" => Some(FILLED_RED.to_string()),
            "fgreen" => Some(FILLED_GREEN.to_string()),
            "fyellow" => Some(FILLED_YELLOW.to_string()),
            "fblue" => Some(FILLED_BLUE.to_string()),
            "fcyan" => Some(FILLED_CYAN.to_string()),
            "fwhite" => Some(FILLED_WHITE.to_string()),
            "fblack" => Some(FILLED_BLACK.to_string()),
            _ => None,
        }
    }
}