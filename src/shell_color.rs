pub mod parser {
    const DELIMITER_START: &str = &"[";
    const DELIMITER_END: &str = &"]";

    const RED: &str = "\x1b[31m";
    const GREEN: &str = "\x1b[32m";
    const YELLOW: &str = "\x1b[33m";
    const BLUE: &str = "\x1b[34m";
    const CYAN: &str = "\x1b[36m";
    const WHITE: &str = "\x1b[37m";
    const RESET: &str = "\x1b[0m";
    
    pub fn tokenize(source: &str) -> Vec<String> {
        let mut tokenized = (0..(source.len() - DELIMITER_START.len().max(DELIMITER_END.len()) + 1)) 
            .fold(vec!["".to_string()], |mut acc, i| {
                if &source[i..(i + DELIMITER_START.len())] == DELIMITER_START {
                    acc.push("".to_string());
                }

                let latest = acc.last_mut();
                
                match latest {
                    Some(s) => {
                        s.push_str(&source[i..i + 1]);
                    },
                    None => panic!("No latest")
                }

                if &source[i..(i + DELIMITER_END.len())] == DELIMITER_END {
                    acc.push("".to_string());
                }

                acc
            })
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>();
        
        if tokenized.len() >= 2 {
            let last = tokenized.last().map(|s| s.clone());

            if let Some(ref s) = last {
                if s.starts_with(DELIMITER_START) && !s.ends_with(DELIMITER_END) {
                    tokenized.remove(tokenized.len() - 1);
                    tokenized.last_mut().unwrap().push_str(last.unwrap().as_str());
                }
            }
        }

        tokenized
    }

     #[test]
    fn test_tokenize() {
        assert_eq!(tokenize(&"r"), vec!["r".to_string()]);
        assert_eq!(tokenize(&"r[g"), vec!["r[g".to_string()]);
        assert_eq!(tokenize(&"[r]"), vec!["[r]".to_string()]);
        assert_eq!(tokenize(&"[r]g"), vec!["[r]".to_string(), "g".to_string()]);
        assert_eq!(tokenize(&"[r]g[r]"), vec!["[r]".to_string(), "g".to_string(), "[r]".to_string()]);
        assert_eq!(tokenize(&"[r]g[r]b"), vec!["[r]".to_string(), "g".to_string(), "[r]".to_string(), "b".to_string()]);
    }
    
    fn is_tag(s: &str) -> bool {
        s.starts_with(DELIMITER_START) && s.ends_with(DELIMITER_END)
    }
    
    #[test]
    fn test_is_tag() {
        assert_eq!(is_tag(&"[r]"), true);
        assert_eq!(is_tag(&"r"), false);
        assert_eq!(is_tag(&"r]"), false);
        assert_eq!(is_tag(&"[r"), false);
    }
    
    pub fn extract_token(token: &str) -> Option<String> {
        match is_tag(token) {
            true => Some(token[1..token.len() - 1].to_string()),
            false => None
        }
    }
    
    #[test]
    fn test_extract_token() {
        assert_eq!(extract_token(&"[r]"), Some("r".to_string()));
        assert_eq!(extract_token(&"[r"), None);
        assert_eq!(extract_token(&"r]"), None);
    }
    
    pub fn convert_color(token: &String) -> Option<String> {
        match &token[..] {
            "red" => Some(RED.to_string()),
            "green" => Some(GREEN.to_string()),
            "yellow" => Some(YELLOW.to_string()),
            "blue" => Some(BLUE.to_string()),
            "cyan" => Some(CYAN.to_string()),
            "white" => Some(WHITE.to_string()),
            "reset" => Some(RESET.to_string()),
            _ => None,
        }
    }

    pub fn convert(source: &str) -> String {
        tokenize(source).iter().map(|token| {
            extract_token(token).and_then(|token| {
                convert_color(&token)
            }).unwrap_or(token.to_string())
        }).collect::<String>()
    }
    
    #[test]
    fn test_convert() {
        assert_eq!(convert(&"r"), "r".to_string());
        assert_eq!(convert(&"[red]g"), "\x1b[31mg".to_string());
    }
}