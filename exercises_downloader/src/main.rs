// Download the exercises from https://rustplatform.com/
// and make them available locally.

// use reqwest::Error;  // DEBUG
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct Bite {
    name: String,
    slug: String,
    description: String,
    level: String,
    template: String,
    libraries: String,
    author: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Download the exercises from Pybites Rust");

    // collect arguments
    let args: Vec<String> = env::args().collect();

    // define the base path from args[0] / exercises
    let base_path = Path::new(&args[0]).parent().unwrap().join("exercises");
    fs::create_dir_all(&base_path)?;

    // send the request
    let client = reqwest::blocking::Client::new();
    let response = client.get("https://rustplatform.com/api/").send()?;
    println!("Status: {}", response.status()); // DEBUG
    println!();

    // just testing, print out the status and headers and exit
    if args.contains(&String::from("--test")) {
        println!("Headers:\n{:#?}", response.headers());
        return Ok(());
    }

    // extract the exercises from the response
    let bites: Vec<Bite> = response.json()?;
    println!("I found {:#?} exercises!", bites.len());
    println!();

    for bite in bites {
        print!("{:#?}", bite.name);
        // do something
        println!(" ✅");
    }
    Ok(())
}
