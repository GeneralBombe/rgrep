use std::error::Error;
use std::fs;
use std::str;
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
                    let _file_path = match args.next() {
                        Some(arg) => arg,
                        None => return Err("Didn't get a file path"),
                    };
                } else {
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


pub fn print_file_content(file_path: &str) -> String {
    let byte_vec = fs::read(file_path).expect("geht nicht");
    let input = String::from_utf8_lossy(&byte_vec).to_string();
    return input;
  
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


pub fn search_string<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}


pub fn run(config: &Config, c_path: &String) -> Result<(), Box<dyn Error>> {
    let contents = print_file_content(&c_path);
    //println!("{}", contents);
    for line in search_string(&config.query, &contents) {
        println!("{c_path}:");
        println!("{line}");
    }

    Ok(())
}