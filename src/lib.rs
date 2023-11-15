use std::error::Error;
use std::fs;
use std::str;
use inline_colorization::*;
use std::io::{self, Write};
use std::borrow::Cow;
pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
    using_option: bool,
    use_recursion: bool
}
#[allow(dead_code)]
impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        let mut ignore_case: bool = false;
        let mut using_option: bool = false;
        let mut use_recursion: bool = false;

        let mut query = String::from("kek");
        let mut file_path = String::from("./");

        args.next();

        let arg1 = match args.next() {
            Some(arg) => arg.chars().collect::<Vec<char>>(),            
            None => return Err("Didn't get a query string"),
        };
        if let Some(first_char) = arg1.get(0) {
            if first_char == &'-' {
                using_option = true;
                for c in  arg1 {
                    if c == 'i' {
                        ignore_case = true;
                    }
                    if c == 'r' {
                        use_recursion = true;
                    }
                }
                query = match args.next() {
                    Some(arg) => arg,
                    None => return Err("No Search String"),
                };
                if use_recursion == false {
                    file_path = match args.next() {
                        Some(arg) => arg,
                        None => return Err("Didn't get a file path"),
                    };
                } else {
                    println!("jdsaiudh");
                    file_path = match args.next() {
                        Some(arg) => arg,
                        None =>  String::from("./")
                    };
                }
                
                
            } else {
                query = arg1.into_iter().collect();
                file_path = match args.next() {
                    Some(arg) => arg,
                    None => return Err("Didn't get a file path"),
                };
            }
        }
        
        
        
        
        
        
        
        Ok(Config {
            query,
            file_path,
            ignore_case,
            using_option,
            use_recursion
        })
    }
    pub fn read(&self) {
        println!("Options: {}", self.using_option);
        println!("Ignore Case: {}", self.ignore_case);
        println!("Use recusion: {}", self.use_recursion);
        println!("Query: {} \nPath: {}", self.query, self.file_path)
    }
    
    pub fn get_query(&self) -> &str {
        &self.query
    }

    pub fn get_file_path(&self) -> &str {
        &self.file_path
    }

    pub fn get_ignore_case(&self) -> bool {
        self.ignore_case
    }

    pub fn get_using_option(&self) -> bool {
        self.using_option
    }

    pub fn get_use_recursion(&self) -> bool {
        self.use_recursion
    }
    
}

pub fn read_file(config: Config) -> Result<(), Box<dyn Error>> {
    let byte_vec = fs::read(config.file_path).expect("geht nicht");
    let input = String::from_utf8_lossy(&byte_vec);
    println!("FileContent: {:?}", input);
    Ok(())
}


pub fn print_file_content(file_path: &str, config: &Config) -> String {
    let byte_vec = fs::read(file_path).expect("geht nicht");
    let input = String::from_utf8_lossy(&byte_vec).to_string();
    if config.ignore_case == true {
        return input.to_lowercase();
    } else {
        return input;
    }
  
}
#[allow(dead_code)]
pub fn print_files() {
    let paths = fs::read_dir("./").unwrap();

    for _path in paths {
       // println!("Name: {}", path.unwrap().path().display())
    }
}




pub fn print_files_t(input_path: Option<String>, config: &Config) -> Result<(), Box<dyn Error>> {
    let path: String = match input_path {
        Some(the_path) => the_path,
        None => String::from("./"),
    };
    let paths = fs::read_dir(path)?;
    /*for path in paths {
        if let Ok(entry) = path {
            println!("{:?}", entry.path());
        }
    } */
    
    for path in paths {
        if let Ok(entry) = path {
            let metadata = fs::metadata(entry.path());
            //println!("{:?}", entry.path());
            if let Ok(meta) = metadata {
                let new_dir = entry.path().display().to_string();
                //println!("Name: {}", new_dir);
                let dir_bool = meta.is_dir();
                if dir_bool {
                    print_files_t(Some(new_dir), config)?; // Returning the result of the recursive call
                } else {

                    run(config, &new_dir)?; // Returning the result of the 'run' function
                }
            } else {
                println!("Failed to get metadata for {:?}", entry.path());
            }
        }
    } 
    Ok(()) // Return Ok(()) at the end of the function
}


pub fn search_string<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<&'a str> {
    let new_query: String;
    if ignore_case {
        new_query = query.to_lowercase();
    } else {
        new_query = String::from(query);
    }

    contents
        .lines()
        .filter(|line| {
            if ignore_case {
                line.to_lowercase().contains(&new_query)
            } else {
                line.contains(&new_query)
            }
        })
        .collect()

}
fn colorize_word_in_string(word: &str, input_string: &str, lowercase: bool) -> String {
    let replace_word: Cow<str> = if lowercase {
        Cow::Owned(word.to_lowercase())
    } else {
        Cow::Borrowed(word)
    };

    input_string.replace(word, &format!("{color_green}{}{color_reset}", replace_word))
}


pub fn run(config: &Config, c_path: &String) -> Result<(), Box<dyn Error>> {
    let _ = io::stdout().flush();
    let contents = print_file_content(&c_path, &config);
    //println!("{}", contents);
    for lineout in search_string(&config.query, &contents, config.ignore_case) {
        let line = colorize_word_in_string(&config.get_query(), lineout, config.get_ignore_case());
        print!("{color_red}{c_path}{color_cyan}:{color_reset}");
        println!("{line}");
        let _ = io::stdout().flush();
    }

    Ok(())
}