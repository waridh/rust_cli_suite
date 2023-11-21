use std::{
    fs,
};


pub struct Config {
    pub query: String,
    pub file_name: String
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_name = args[2].clone();

        Config {query, file_name}
    }
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            Err::<Config, &'static str>("Not enough arguments")
        } else {
            let query = args[1].clone();
            let file_name = args[2].clone();

            Ok(Config {query, file_name})
        }
    }
    pub fn read_file(&self) -> String {
        fs::read_to_string(&self.file_name).expect("Expected file")
    }
}
