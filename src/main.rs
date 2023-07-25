use std::time::Duration;
use std::error::Error;
use std::path::Path;
use std::thread;
use reqwest::header::{ HeaderValue, HeaderMap, COOKIE };
use reqwest;
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
    dbg!("Importing Fetch.toml file");
    match use_config() {
        Ok(config) => return Ok(config),
        Err(error) => {
            dbg!("Could find Fetch.toml file, trying to create it. {}", error);
            try_create_config_file();
            return use_config();
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

fn main() -> Result<(), Box<dyn Error>> {
    // Import config
    let config_file = include_config()?;
    let config = config_file.get("configuration").expect("No configuration in Fetch.toml file");

    // Segregate config values
    let year = config
        .get("year").expect("No year defined on Fetch.toml")
        .as_str().expect("Bad year definition on Fetch.toml");
    let path = config
        .get("path").expect("No path defined on Fetch.toml")
        .as_str().expect("Bad path definition on Fetch.toml");
    let extension = config
        .get("extension").expect("No extension defined on Fetch.toml")
        .as_str().expect("Bad extension definition on Fetch.toml");
    let session = config
        .get("session").expect("No session was found on config file Fetch.toml")
        .as_str().expect("Bad cookies definition on Fetch.toml");
    
    // Default session header
    let mut default_headers = HeaderMap::new();
    let cookie_value = format!("session={};", session);
    default_headers.insert(COOKIE, HeaderValue::from_str(cookie_value.as_str()).expect("Bad cookie header value"));

    // Blocking Client
    let client = reqwest::blocking::Client::builder()
        .default_headers(default_headers)
        .build()?;

    // Days loop
    for day in (1..26).map(|n| n.to_string()) {
        println!("Fetching day {}...", day);

        let prompt_url = format!("https://adventofcode.com/{}/day/{}", year, day).parse::<reqwest::Url>()?;
        let input_url  = format!("https://adventofcode.com/{}/day/{}/input", year, day).parse::<reqwest::Url>()?;

        thread::sleep(REQUEST_DELAY);
        let input_response = client.get(input_url).send()?;
        if !input_response.status().is_success() { panic!("{:?}", &input_response); }; 
        let input_response_text = input_response.text()?;
        let input_path = format!("{}/{}.{}", &path, day, extension);

        thread::sleep(REQUEST_DELAY);
        let prompt_response = client.get(prompt_url).send()?;
        if !prompt_response.status().is_success() { panic!("{:?}", &prompt_response); }; 
        let prompt_response_text = prompt_response.text()?;
        let prompt_path = format!("{}/prompt/{}.{}", &path, day, extension);

        if !Path::new(path).exists() { std::fs::create_dir(&path)?; };

        let prompt_dir = format!("{}/prompt", path);
        if !Path::new(&prompt_dir).exists() { std::fs::create_dir(&prompt_dir)?; };

        std::fs::write(input_path, input_response_text)?;
        std::fs::write(prompt_path, prompt_response_text)?;
    }

    return Ok(());
}
