use rgrep::Config;
use std::env;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err); // Changed {err} to {} within the string
        std::process::exit(1);
    });
    config.read();
    //if let Err(e) = rgrep::print_files_t(None, &config) {
      //  eprint!("Error: {}", e); // Changed {e} to {} within the string
    //}

    // Rest of your code remains unchanged...
}
