//to read the String lines as user inputs
use std::env;
//to read and write file
use std::fs;
//to exit the program without panic
use std::process;
//to be able to use Error type
use std::error::Error;



fn main() {
    //if the user pass no args on terminal, it retruns a string vector with the file path on first index
    //if user write //cargo run something, the "something" added to vector with next index
    //Strings written with spaces between them are added to the vector in order.
    let args: Vec<String> = env::args().collect();

    //parsing the args as Result Enum,
    // if its ok, passing the value to variable, if its Err executing opp function
    let my_config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem Parsing Arguments: {}", err);
        process::exit(1)
    });

    //if there is no error in run() the program will end so we dont need to handle Ok() result
    if let Err(e) = run(my_config){
        println!("Application Error : {}", e);
        process::exit(1);
    }

   
}

fn run(config: Config) -> Result<(), Box<dyn Error>>{

    //trying to read the content of the file which determine with filename by user
    //if an error accured while reading '?' returns Error automaticly 
    let contents = fs::read_to_string(config.file_name)?;

    println!("User Input is {}", contents);
    //if no error return an empty Ok() result enum
    Ok(())
}


//a simple struct to keep args
struct Config {
    query : String,
    file_name : String,
}

impl Config{
    //adding a method to Config struct as constractor
     
    fn new(args:&[String])-> Result<Config, &str> {
        //We check that the necessary arguments have been passed
        //if is there a missing argument it returns Result Err
        if args.len() < 3 {
           return Err("not enough arguments");
        }

        //we dont want to take the ownership of the Strings so we will use .clone()
        let query = args[1].clone(); 
        let file_name = args[2].clone();

        //creating a Config struct instance and return with result enum Ok(Config)
        Ok(Config{ query : query, file_name : file_name})

}
}




