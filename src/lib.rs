use std::fs;
use std::error::Error;
use std::env;
// config that occurs after parsing arguments from CLI
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err("Not enough args".to_string())
        } 
        let query = &args[1].clone();
        let filename = &args[2].clone();

        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        Ok(Config { query: query.to_string() , filename: filename.to_string(), case_sensitive })
    }    
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    
    let results =if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_sensitive(&config.query, &contents)  
    };
    for line in results {
        println!("{}", line)
    }
    
    Ok(())
}
// this function will need to implement lifetimes
// contents will only have lifetime of this function in order to search
// need lifetimes here to reference borrowing
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query) )
        .collect()
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut t = Vec::new();
    
    for i in contents.lines() {
       // println!(" new line{}", i);
        if i.to_lowercase().contains(&query) {
            t.push(i);
            //println!("{:?}", t)
        }
        
    }
    t
}

// test configuration 
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn initial() {
        let query_test = "chain";
        let contents_test = 
        "rust is a great lang to learn
it has memory safe concepts
it is performant
used in blockchain";
        assert_eq!(
            vec!["used in blockchain"],
            search(&query_test, &contents_test)
        )

    }
}