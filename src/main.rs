use std::{env, process};

use minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("query is:{}", config.query);
    println!("filename is:{}", config.filename);

    if let Err(err) = run(&config){
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    }
}