use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    // Warning: this will panic if we aren't using valid Unicode characters!
    let args: Vec<String> = env::args().collect();
    
    // Because Config::new returns a Config instance, we use unwrap_or_else
    // This is because we want to assign config a Config instance,
    //   or we want to handle any error that occurs in this process
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing args: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In {}", config.filename);

    // Because we only care whether run returns an error, we don't need
    //   to unwrap it, thus we use this 'if let' pattern
    // run only returns the unit "()" type on success
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    // Takes in a pointer to a String array (ie, the arguments passed within the cli)
    // Returns a new Config instance upon success, or an Err(&str)
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
    
        Ok(Config {query, filename})
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}