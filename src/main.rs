use std::fs::read_to_string;
use std::time::Duration;
use std::error::Error;
use std::path::Path;
use std::io::Write;
use std::thread;
use std::env;
use exitcode;
use reqwest;
use reqwest::header::{ HeaderValue, HeaderMap, COOKIE };
use scraper;
use toml;

/// The default configuration .toml file contents
const DEFAULT_CONFIG: &str = "[configuration]\nyear=2015\npath=\"input\"\nextension=\"txt\"\nsession=\"\"";

/// Default request delay (to avoid rate limitting)
const REQUEST_DELAY: Duration = Duration::from_millis(500);

/// Returns the config table
///
/// if unable to read from the file, tries to create it, and if the method
/// also fails to create a file, it throws a warn and load defaults from memory
fn include_config() -> Result<toml::Table, Box <dyn Error>> {
    match use_config() {
        Ok(config) => return Ok(config),
        Err(error) => {
            if try_create_config_file() { return use_config(); }
            else { return Err(error) }
        }
    }
}

/// Try to read the file Fetch.toml
fn use_config() -> Result<toml::Table, Box<dyn Error>> {
    let config = std::fs::read_to_string("Fetch.toml")?;
    let table = config.parse::<toml::Table>()?;
    return Ok(table);
}

/// Try to create the file Fetch.toml
fn try_create_config_file() -> bool { 
    let did_create = std::fs::write("Fetch.toml", DEFAULT_CONFIG).is_ok();
    return did_create;
}

fn run(day: Option<&String>) -> Result<(), Box<dyn Error>> {
    // Import config
    let config_file = include_config()?;
    let config = config_file.get("configuration").expect("No configuration in Fetch.toml file");

    // Segregate config values
    let year = config
        .get("year").expect("No year defined on Fetch.toml")
        .as_integer().expect("Bad year definition on Fetch.toml").to_string();
    let path = config
        .get("path").expect("No path defined on Fetch.toml")
        .as_str().expect("Bad path definition on Fetch.toml");
    let extension = config
        .get("extension").expect("No extension defined on Fetch.toml")
        .as_str().expect("Bad extension definition on Fetch.toml");
    let session = config
        .get("session").expect("No session was found on config file Fetch.toml")
        .as_str().expect("Bad cookies definition on Fetch.toml");

    // Check if session cookie existst
    if session.is_empty() { Err("No session cookie in Fetch.toml file!")? };
    
    // Default session header
    let mut default_headers = HeaderMap::new();
    let cookie_value = format!("session={};", session);
    default_headers.insert(COOKIE, HeaderValue::from_str(cookie_value.as_str()).expect("Bad cookie header value"));

    // Blocking Client
    let client = reqwest::blocking::Client::builder()
        .default_headers(default_headers)
        .build()?;

    // Check or create dirs
    if !Path::new(path).exists() { std::fs::create_dir(&path)?; };
    let prompt_dir = format!("{}/prompt", path);
    if !Path::new(&prompt_dir).exists() { std::fs::create_dir(&prompt_dir)?; };

    // Days loop
    let days = match day {
        Some(day) => vec![day.clone()],
        None => (1..26).map(|d| d.to_string()).collect::<Vec<String>>(),
    };
    for day in days {
        print!("Fetching day {}... ", day);
        std::io::stdout().flush()?;

        let prompt_path = format!("{}/prompt/{:0>2}.md", &path, day);
        let prompt_exists = Path::new(&prompt_path).exists() && read_to_string(&prompt_path).map(|f| f.find("id=\"part2\"")).is_ok();
        if !prompt_exists {
            let prompt_url = format!("https://adventofcode.com/{}/day/{}", year, day).parse::<reqwest::Url>()?;
            thread::sleep(REQUEST_DELAY);
            let prompt_response = client.get(prompt_url).send()?;
            if !prompt_response.status().is_success() { Err("Unexpected error while fetching prompt")? }; 
            let prompt_response_text = prompt_response.text()?;
            let document = scraper::Html::parse_document(&prompt_response_text);
            let selector = &scraper::Selector::parse("article")?;
            let articles = document.select(selector);
            let prompt_text = articles.map(|a| a.inner_html()).collect::<Vec<String>>().join("\n");
            std::fs::write(prompt_path, prompt_text)?;
        }

        let input_path = format!("{}/{:0>2}.{}", &path, day, extension);
        let input_exists = Path::new(&input_path).exists();  
        if !input_exists {
            let input_url  = format!("https://adventofcode.com/{}/day/{}/input", year, day).parse::<reqwest::Url>()?;
            thread::sleep(REQUEST_DELAY);
            let input_response = client.get(input_url).send()?;
            if !input_response.status().is_success() { Err("Unexpected error while fetching input (maybe check your session cookie?)")? }; 
            let input_response_text = input_response.text()?;
            std::fs::write(input_path, input_response_text)?;
        }

        match (prompt_exists, input_exists) {
            (false, false) => println!("Done!"),
            (true, false)  => println!("Skipped existent prompt , Done!"),
            (false, true)  => println!("Skipped existent input, Done!"),
            (true, true)  => println!("Skipped existent prompt and input, Done!"),
        }
    }

    return Ok(());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args.get(1);
    println!("Running aoc input fetcher");
    match run(day) {
        Ok(()) => {
            println!("Finished fetching files!");
            std::process::exit(exitcode::OK);
        },
        Err(error) => {
            eprintln!("Unexpected error! {}", error);
            std::process::exit(exitcode::DATAERR)
        }
    }
}
