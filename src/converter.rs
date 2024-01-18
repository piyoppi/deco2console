pub mod converter {
    use crate::tokenizer::tokenizer;
    use crate::token_converter::token_converter;
    use std::collections::HashMap;
    
    pub fn convert(source: &str, converter: HashMap<String, Box<token_converter::Converter>>) -> String {
        tokenizer::tokenize(source).iter().map(|token| {
            tokenizer::extract_token(token).and_then(|token| {
                converter.get(&token).and_then(|converter| converter(&token))
            }).unwrap_or(token.value.to_string())
        }).collect::<String>()
    }
    
   
    #[test]
    fn test_convert() {
        assert_eq!(
            convert(
                &"r",
                vec![
                    ("[red]".to_string(), token_converter::boxed_fn(|_| Some("r".to_string())))
                ].into_iter().collect()
            ),
            "r".to_string()
        );
        assert_eq!(
            convert(
                &"[red]g",
                vec![
                    ("red".to_string(), token_converter::boxed_fn(|_| Some("\x1b[31m".to_string())))
                ].into_iter().collect()
            ),
            "\x1b[31mg".to_string()
        );
    }
}