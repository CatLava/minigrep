use std::env;
use std::fs;
use std::error::Error;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args).unwrap();

    println!("query = {}", config.query);
    println!("filename = {}", config.filename);

    run(config);

  

    
    // create a parse command function for query and filename
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err("Not enough args".to_string())
        } 
        let query = &args[1].clone();
        let filename = &args[2].clone();

        Ok(Config { query: query.to_string() , filename: filename.to_string() })
    }    
}

fn run(config: Config) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    
    println!("Text of tile:\n{}", contents);
    Ok(contents)
}

