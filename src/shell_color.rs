pub mod parser {
    const RED: &str = "\x1b[31m";
    const GREEN: &str = "\x1b[32m";
    const YELLOW: &str = "\x1b[33m";
    const BLUE: &str = "\x1b[34m";
    const CYAN: &str = "\x1b[36m";
    const WHITE: &str = "\x1b[37m";
    const RESET: &str = "\x1b[0m";

    pub fn convert(source: &String) {
        let converted = source.chars().map(|c| {
            let cc = match c {
                'r' => RED.to_string(),
                'g' => GREEN.to_string(),
                'y' => YELLOW.to_string(),
                'b' => BLUE.to_string(),
                'c' => CYAN.to_string(),
                'w' => WHITE.to_string(),
                'R' => RESET.to_string(),
                _ => "".to_string(),
            };
            
            if cc.is_empty() {
                c.to_string()
            } else {
                cc
            }
        }).collect::<String>();
        
        println!("{}", converted)
    }
}