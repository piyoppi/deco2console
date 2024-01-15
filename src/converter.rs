pub mod converter {
    use crate::tokenizer::tokenizer;
    use std::collections::HashMap;
    
    type Converter = fn(&str) -> Option<String>;

    pub fn convert(source: &str, converter: HashMap<String, Converter>) -> String {
        tokenizer::tokenize(source).iter().map(|token| {
            tokenizer::extract_token(token).and_then(|token| {
                converter.get(&token).and_then(|converter| {
                    Some(converter(&token))
                })
            }).unwrap_or(token.value.to_string())
        }).collect::<String>()
    }
    
    #[test]
    fn test_convert() {
        assert_eq!(
            convert(
                &"r",
                vec![
                    ("[red]".to_string(), crate::shell_color::shell_color::convert_color as Converter)
                ].into_iter().collect()
            ),
            "r".to_string()
        );
        assert_eq!(
            convert(
                &"[red]g",
                vec![
                    ("[red]".to_string(), crate::shell_color::shell_color::convert_color as Converter)
                ].into_iter().collect()
            ),
            "\x1b[31mg".to_string()
        );
    }
}