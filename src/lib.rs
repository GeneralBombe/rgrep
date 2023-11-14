use std::error::Error;
use std::fs;
use std::str;
pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
    using_option: bool,
    use_recusion: bool
}
#[allow(dead_code)]
impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        let mut ignore_case: bool = false;
        let mut using_option: bool = false;
        let mut use_recusion: bool = false;

        let mut query = String::from("kek");
        let _file_path = String::from("kek");

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
                        use_recusion = true;
                    }
                }
                let _query = match args.next() {
                    Some(arg) => arg,
                    None => return Err("No Search String"),
                };
                if use_recusion == false {
                    let _file_path = match args.next() {
                        Some(arg) => arg,
                        None => return Err("Didn't get a file path"),
                    };
                }
                
                
            } else {
                query = arg1.into_iter().collect();
                let _file_path = match args.next() {
                    Some(arg) => arg,
                    None => return Err("Didn't get a file path"),
                };
            }
        }
        
        
        
        
        
        
        println!("{}", query);
        let query = String::from("3");
        let file_path = String::from("./test.txt");
        Ok(Config {
            query,
            file_path,
            ignore_case,
            using_option,
            use_recusion
        })
    }
    pub fn read(&self) {
        println!("Options: {}", self.using_option);
        println!("Ignore Case: {}", self.ignore_case);
        println!("Use recusion: {}", self.use_recusion);
        println!("Query: {} \nPath: {}", self.query, self.file_path)
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
    
    for path in paths {
        if let Ok(entry) = path {
            let metadata = fs::metadata(entry.path());
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
    //println!("Search called");

    contents.lines().filter(|line| line.contains(query)).collect()
}


pub fn run(config: &Config, c_path: &String) -> Result<(), Box<dyn Error>> {
    let contents = print_file_content(&c_path);

    for line in search_string(&config.query, &contents) {
        println!("{c_path}:");
        println!("{line}");
    }

    Ok(())
}