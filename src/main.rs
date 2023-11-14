use rgrep::Config;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "0");
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err); // Changed {err} to {} within the string
        std::process::exit(1);
    });
    config.read();
    let recursion = config.get_use_recursion();
    let ignore_case = config.get_ignore_case();
    if recursion {
      let f_p: String = config.get_file_path().to_string();
      if let Err(e) = rgrep::print_files_t(Some(f_p), &config) {
        println!("Error: {}", e); // Changed {e} to {} within the string
      }

    } else {
      if let Err(e) = rgrep::run(&config, &config.get_file_path().to_string()) {
        println!("Error: {}", e); // Changed {e} to {} within the string

      }
    }
    
    //if let Err(e) = rgrep::print_files_t(None, &config) {
      //  eprint!("Error: {}", e); // Changed {e} to {} within the string
    //}

    // Rest of your code remains unchanged...

}
