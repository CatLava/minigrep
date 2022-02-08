use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args).unwrap();

    println!("query = {}", config.query);
    println!("filename = {}", config.filename);

    if let Err(e) = minigrep::run(config){
        println!("Err in processesing: {}", e);
        process::exit(1);
    };
}


