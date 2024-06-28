use walkdir::WalkDir;
use std::error::Error;
use std::fs;
use std::env;
pub fn run(config:Config)-> Result<(), Box<dyn Error>>{
    let mut i:i64;
    let mut loading_displayed = false;
    i=0;
    for entry in WalkDir::new(config.file_path).into_iter().filter_map(Result::ok).filter(|e| e.file_type().is_file()){
        match fs::read_to_string(entry.path()) {

            Ok(contents) => {
                let results = search_case_insensitive(&config.query, &contents);
                for line in results {
                    i+=1;
                    println!();
                    println!("{i} : {}'{line}' : {}{}\x1b[0m.","\x1b[32m","\x1b[33m", entry.path().display());
                    println!();
                    loading_displayed = false;
                }
            }
            Err(_) => {
                if !loading_displayed {
            println!("Loading..."); // Display loading message only once
            loading_displayed = true; // Set flag to true to indicate loading message displayed
        }
              continue;
            }
        }
}


    Ok(())

    }

    fn search_case_insensitive<'a>(query: &str, contents : &'a str)->Vec<&'a str>{
        let query=query.to_lowercase();
        let mut results=Vec::new();

        for line in contents.lines(){
            if line.to_lowercase().contains(&query){
                results.push(line);
            }
        }
        results
    }


pub struct Config {
    pub query:String,
    pub file_path:String,
    pub ignore_case: bool,
}

impl Config{
    pub fn build(args:&[String])->Result<Config, & 'static str>{

        if args.len()<3{
            return Err("not enough argumnets");
        }

        let query=args[1].clone();
        let file_path =args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config{query, file_path, ignore_case,})

    }
}

