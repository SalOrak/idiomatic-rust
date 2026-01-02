use serde::{Deserialize};
use toml;
use anyhow;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Config {
    title: String,
    width: i32,
    height: i32,
    should_open: Option<bool>,
    tail: bool
}


#[allow(unused_doc_comments)]
/// The "normal" way would be to do it in sequence.
fn normal_get_config(path: &str) -> anyhow::Result<()> {

    /// First we read the file in bytes
    let raw_config = std::fs::read(path)?;

    /// Then we parse it as utf8
    let config_str = String::from_utf8(raw_config)?;

    // Then we deserialize it
    let config: Config = toml::from_str(&config_str)?;

    // Then we do some work on it
    do_some_work(config);

    Ok(())
}

#[allow(unused_doc_comments)]
/// The problem with `normal_get_config` is that the function gets populated with
/// a lot of different variables that are only used to work with the configuration file.
/// By the end of the function, we only work with the final Config file.
/// A better way to do this is make use of the `block pattern`.
fn idiomatic_get_config(path: &str) -> anyhow::Result<()> {


    /// Everything related to the `config` part is now abstracted into a single statement.
    /// All the other variables are dropped once we have the final Config object.
    let config = {
        let raw_config = std::fs::read(path)?;
        let config_str = String::from_utf8(raw_config)?;
        toml::from_str(&config_str)?
    };

    /// Now it is impossible to provide the incorrect value here, as there is only a single 
    /// variable in the function stack: the config: Config one. 
    do_some_work(config);

    Ok(())
}


fn do_some_work(config: Config) {
    println!("I'm doing some work on {:?}", config);
    println!("Work on -> {}", config.title);
    println!("Total space: {}", config.width * config.height);
    if config.should_open.is_some() {
        println!("Does it open by default? {}", config.should_open.unwrap());
    }
}

#[allow(unused_doc_comments)]
fn main() {
    /// Let's imagine we want to initialize a new Configuration object.
    /// For that, first we need to read the file where the configuration lives
    /// After reading the file, we need to parse it into a Config file then we calculate
    /// some things based on the config file.

    let path = "block.toml";

    {
         if let Err(err) =  normal_get_config(path) {
             println!("{:?}", err);
         }
         if let Err(err) =  idiomatic_get_config(path) {
             println!("{:?}", err);
         }
    }
}
