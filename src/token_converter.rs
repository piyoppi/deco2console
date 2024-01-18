pub mod token_converter {
    pub type Converter = dyn Fn(&str) -> Option<String>;
    
    pub fn boxed_fn<F>(f: F) -> Box<dyn Fn(&str) -> Option<String>>
    where
        F: Fn(&str) -> Option<String> + 'static,
    {
        Box::new(f)
    }
}