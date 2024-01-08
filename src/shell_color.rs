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
    
    #[derive(PartialEq, Debug, Clone)]
    enum TokenKind {
        Element,
        Text,
    }

    #[derive(PartialEq, Debug, Clone)]
    pub struct Token {
        value: String,
        kind: TokenKind,
    }

    pub fn tokenize(source: &str) -> Vec<Token> {
        let mut tokenized = (0..(source.len() - DELIMITER_START.len().max(DELIMITER_END.len()) + 1)) 
            .fold(vec![Token {value: "".to_string(), kind: TokenKind::Text}], |mut acc, i| {
                if &source[i..(i + DELIMITER_START.len())] == DELIMITER_START {
                    acc.push(Token {value: "".to_string(), kind: TokenKind::Element});
                }

                let latest = acc.last_mut();
                
                match latest {
                    Some(s) => {
                        s.value.push_str(&source[i..i + 1]);
                    },
                    None => panic!("No latest")
                }

                if &source[i..(i + DELIMITER_END.len())] == DELIMITER_END {
                    acc.push(Token {value: "".to_string(), kind: TokenKind::Text});
                }

                acc
            })
            .into_iter()
            .filter(|s| !s.value.is_empty())
            .collect::<Vec<Token>>();
        
        if tokenized.len() >= 2 {
          let last = tokenized.last().map(|s| s.clone());

          if let Some(s) = last {
            if s.value.starts_with(DELIMITER_START) && !s.value.ends_with(DELIMITER_END) {
              tokenized.remove(tokenized.len() - 1);
              tokenized.last_mut().unwrap().value.push_str(&s.value);
            }
          }
        }

        tokenized
    }

     #[test]
    fn test_tokenize() {
        assert_eq!(tokenize(&"r"), vec![Token {value: "r".to_string(), kind :TokenKind::Text}]); 
        assert_eq!(tokenize(&"[r]"), vec![Token {value: "[r]".to_string(), kind :TokenKind::Element}]);
        assert_eq!(tokenize(&"[r]g"), vec![Token {value: "[r]".to_string(), kind :TokenKind::Element}, Token {value: "g".to_string(), kind :TokenKind::Text}]);
        assert_eq!(tokenize(&"[r]g[r]"), vec![Token {value: "[r]".to_string(), kind :TokenKind::Element}, Token {value: "g".to_string(), kind :TokenKind::Text}, Token {value: "[r]".to_string(), kind :TokenKind::Element}]);
        assert_eq!(tokenize(&"[r]g[r]b"), vec![Token {value: "[r]".to_string(), kind :TokenKind::Element}, Token {value: "g".to_string(), kind :TokenKind::Text}, Token {value: "[r]".to_string(), kind :TokenKind::Element}, Token {value: "b".to_string(), kind :TokenKind::Text}]);
        assert_eq!(tokenize(&"r[g"), vec![Token {value: "r[g".to_string(), kind :TokenKind::Text}]);
    }
    
    fn is_tag(s: &Token) -> bool {
        s.kind == TokenKind::Element
    }
    
    #[test]
    fn test_is_tag() {
        assert_eq!(is_tag(&Token {value: "r".to_string(), kind :TokenKind::Text}), false);
        assert_eq!(is_tag(&Token {value: "[r]".to_string(), kind :TokenKind::Element}), true);
    }
    
    pub fn extract_token(token: &Token) -> Option<String> {
        match is_tag(token) {
            true => Some(token.value[1..token.value.len() - 1].to_string()),
            false => None
        }
    }
    
    #[test]
    fn test_extract_token() {
        assert_eq!(extract_token(&Token {value: "r".to_string(), kind :TokenKind::Text}), None);
        assert_eq!(extract_token(&Token {value: "[r]".to_string(), kind :TokenKind::Element}), Some("r".to_string()));
        assert_eq!(extract_token(&Token {value: "[red]".to_string(), kind :TokenKind::Element}), Some("red".to_string()));
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
            }).unwrap_or(token.value.to_string())
        }).collect::<String>()
    }
    
    #[test]
    fn test_convert() {
        assert_eq!(convert(&"r"), "r".to_string());
        assert_eq!(convert(&"[red]g"), "\x1b[31mg".to_string());
    }
}