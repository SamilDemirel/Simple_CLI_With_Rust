//to read and write file
use std::fs;
//to be able to use Error type
use std::error::Error;
//to use var
use std::env;



pub fn run(config: Config) -> Result<(), Box<dyn Error>>{

    //trying to read the content of the file which determine with filename by user
    //if an error accured while reading '?' returns Error automaticly 
    let contents = fs::read_to_string(config.file_name)?;
        
    let results = if !config.case_insensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    //we are calling search and it retruns a vector, then we loop over vector and print all line in it
    for line in results{
        println!("{}", line);
    }

    //if no error return an empty Ok() result enum
    Ok(())
}


//a simple struct to keep args
pub struct Config {
    pub query : String,
    pub file_name : String,
    pub case_insensitive : bool,
}

impl Config{
    //adding a method to Config struct as constractor
     
    pub fn new(args:&[String])-> Result<Config, &str> {
        //We check that the necessary arguments have been passed
        //if is there a missing argument it returns Result Err
        if args.len() < 3 {
           return Err("not enough arguments");
        }

        //we dont want to take the ownership of the Strings so we will use .clone()
        let query = args[1].clone(); 
        let file_name = args[2].clone();

        //creating a variable "CASE_INSENSITIVE" and setting it on command line true
        //if variable does not set is_err retruns false
        //the command to set variable is:
            //export CASE_SENSETIVE=true
            //then run the program
        let case_insensitive = env::var("CASE_INSENSITIVE").is_err();
 
        //creating a Config struct instance and return with result enum Ok(Config)
        Ok(Config{ query : query, file_name : file_name, case_insensitive: case_insensitive})

    }
}
//we have to define life time
//in this case, return's lifetime should bound to contents parameter
pub fn search<'a>(query: & str, contents:&'a str) -> Vec<&'a str>{

    let mut results = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>( query: & str, contents:&'a str) -> Vec<&'a str>{
   
   let query = query.to_lowercase(); //it returns a new String, does not modify original String

   let mut results = Vec::new();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    
    results

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "Rust: safe, fast, productive.";

        assert_eq!(vec!["safe, fast"], search(query, contents));
                            
    }

}
