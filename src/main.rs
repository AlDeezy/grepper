use std::env;
use std::process;

use grepper::Config;

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

    // Because we only care whether run returns an error, we don't need
    //   to unwrap it, thus we use this 'if let' pattern
    // run only returns the unit "()" type on success
    if let Err(e) = grepper::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}