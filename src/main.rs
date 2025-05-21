//to read the String lines as user inputs
use std::env;
//to exit the program without panic
use std::process;
//to use the struct in lib.rs
use minigrep::Config;


fn main() {
    //if the user pass no args on terminal, it retruns a string vector with the file path on first index
    //if user write //cargo run something, the "something" added to vector with next index
    //Strings written with spaces between them are added to the vector in order.
    let args: Vec<String> = env::args().collect();

    //parsing the args as Result Enum,
    // if its ok, passing the value to variable, if its Err executing opp function
    let my_config = Config::new(&args).unwrap_or_else(|err|{
        
        //eprintline macro print the errors to screen if /cargo run world my_text_file.txt > output.txt command runs
        //if no error, this command prints the output to the file
        eprintln!("Problem Parsing Arguments: {}", err);
        process::exit(1)
    });

    //if there is no error in run() the program will end so we dont need to handle Ok() result
    if let Err(e) = minigrep::run(my_config){ // because the function is in the lib.rs
        eprintln!("Application Error : {}", e);
        process::exit(1);
    }

   
}





