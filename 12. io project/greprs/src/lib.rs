use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct Config {
    pub query : String,
    pub filename : String,
    pub case_sensitive : bool,
}

impl Config{
    pub fn new(mut args : std::env::Args) -> Result<Config, &'static str> {
        //first args is program name
        args.next();

        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {query, filename, case_sensitive})
    }
}

pub fn run(config : Config) -> Result<(), Box<dyn Error>>{
    let mut f = File::open(config.filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results{
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        )
    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    contents.lines()
        .filter(|line| line.contains(query))
        .collect()


    /*
    //filter를 이용해 반복자에 조건을 걸 수 있고, 조건에 걸린 iter를 collect를 통해 vec로 받는다
    
    //other way

    let mut results = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results

    */
}


pub fn search_case_insensitive<'a>(query : &str, contents : &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
} 