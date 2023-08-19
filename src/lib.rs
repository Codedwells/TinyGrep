use std::fs;

pub struct Config {
    pub query: String,
    pub file_name: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        match args.len() {
            1 => Err("Please add a text you want to search."),
            2 => Err("Please add file you want to search."),
            _ => {
                let query = String::from(&args[1]);
                let file_name = String::from(&args[2]);

                Ok(Config { query, file_name })
            }
        }
    }
}

pub fn search_text(config: &Config) -> Result<Vec<String>, &str> {
    let file_text = fs::read_to_string(&config.file_name);

    match file_text {
        Ok(text) => {
            let mut matched = vec![];

            for line in text.lines() {
                if line.contains(&config.query) {
                    matched.push(line.to_string())
                }
            }

            if matched.is_empty() {
                return Err("No line has your query");
            } else {
                return Ok(matched);
            }
        }
        Err(_) => Err("Error while reading the file: Please check if it exists"),
    }
}
