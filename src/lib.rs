use std::fs;
use std::error::Error;

// config that occurs after parsing arguments from CLI
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err("Not enough args".to_string())
        } 
        let query = &args[1].clone();
        let filename = &args[2].clone();

        Ok(Config { query: query.to_string() , filename: filename.to_string() })
    }    
}

pub fn run(config: Config) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    
    println!("Text of tile:\n{}", contents);
    Ok(contents)
}
// this function will need to implement lifetimes
// contents will only have lifetime of this function in order to search
// need lifetimes here to reference borrowing
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut t = Vec::new();
    for i in contents.lines() {
       // println!(" new line{}", i);
        if i.contains(query) {
            t.push(i);
            println!("{:?}", t)
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