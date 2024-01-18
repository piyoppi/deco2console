pub mod tokenizer {
    const DELIMITER_START: &str = &"[";
    const DELIMITER_END: &str = &"]";

    #[derive(PartialEq, Debug, Clone)]
    pub enum TokenKind {
        Element,
        Text,
    }

    #[derive(PartialEq, Debug, Clone)]
    pub struct Token {
        pub value: String,
        pub kind: TokenKind,
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
}