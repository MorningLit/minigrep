use std::{env, error::Error, fs};

#[derive(Debug)]
pub struct Config {
    filepath: String,
    search_str: String,
    ignore_case: bool,
}
impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let filepath = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get filepath string"),
        };
        let search_str = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get query string"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        return Ok(Config {
            filepath,
            search_str,
            ignore_case,
        });
    }
}
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filepath)?;
    for line in search(&config.search_str, &contents, config.ignore_case) {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, is_insensitive: bool) -> Vec<&'a str> {
    let lower = query.to_lowercase();
    if is_insensitive {
        return contents
            .lines()
            .filter(|line| line.to_lowercase().contains(&lower))
            .collect();
    }
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, false)
        );
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search(query, contents, true));
    }
}
