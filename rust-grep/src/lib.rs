use std::{
    env,
    fs,
    error::Error,
};


pub struct Config { // Struct for the user input
    pub query: String,
    pub file_name: String,
    pub ignore_case: bool
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            Err::<Config, &'static str>("Not enough arguments")
        } else {
            // Need to make copies, since it cannot take ownership.
            let query = args[1].to_owned();
            let file_name = args[2].to_owned();
            
            let ignore_case = env::var("IGNORE_CASE").is_ok();

            Ok(Config {query, file_name, ignore_case})   // Can use the constructor
        }
    }
    pub fn read_file(&self) -> String {
        fs::read_to_string(&self.file_name).unwrap()
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = config.read_file();   // Method for reading the file
    let query = config.query;

    // Using iteration to print out all the collected search
    let string_matches = if config.ignore_case {
        search_case_insensitive(&query, &content)
    } else {
        search(&query, &content)
    };

    string_matches.into_iter().for_each(|ele| println!("{ele}"));

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    // This method is the searching
    let mut ret: Vec<&str> = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            ret.push(line);
        }
    };
    ret
}

/// This function will retrive the lines that have the specified substring
/// query present.
pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut ret: Vec<&str> = Vec::new();
    let query = query.to_lowercase();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            ret.push(line);
        }
    };
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    static CONTENT: &str = "\
Rust:
I like to walk.
I like to eat food.
I like cheese
Trust me!
        ";


    #[test]
    fn case_sensitive(){
        let query = "foo";
        assert_eq!(vec!["I like to eat food."], search(query, CONTENT));
    }

    #[test]
    fn case_insensitive() {
        let query = "RuSt";
        assert_eq!(vec!["Rust:", "Trust me!"], search_case_insensitive(
                query, CONTENT));
    }
}
