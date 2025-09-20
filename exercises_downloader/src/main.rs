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

fn write_toml(slug: &String, libraries: String) {
    dbg!(&slug);
    dbg!(&libraries);
}

fn write_exercise(template: String) {
    dbg!(&template);
}
fn write_markdown(slug: &String, description: String, author: String) {
    dbg!(&slug);
    dbg!(&description);
    dbg!(&author);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("Downloading the exercises from Pybites Rust ...");
    let client = reqwest::blocking::Client::new();
    let response = client.get("https://rustplatform.com/api/").send()?;
    println!(" ✅");
    println!();

    // collect the arguments
    let args: Vec<String> = env::args().collect();
    // just testing, print out the status and headers and exit
    if args.contains(&String::from("--test")) {
        println!("Status: {}", response.status());
        println!("Headers:\n{:#?}", response.headers());
        return Ok(());
    }

    // extract the exercises from the response
    let bites: Vec<Bite> = response.json()?;
    println!("I found {:#?} exercises!", bites.len());
    println!();

    // define the base path (args[0] / exercises)
    let base_path = Path::new(&args[0]).parent().unwrap().join("exercises");
    fs::create_dir_all(&base_path)?;

    for bite in bites {
        print!("{:#?}", bite.name);
        let slug = bite.slug;
        let exercise_path = &base_path.join(&bite.level).join(&slug);

        // create the directory and the exercise files (toml, rs)
        fs::create_dir_all(&exercise_path)?;
        // just re-write/update the toml and the md file but make
        // a backup copy of the exercise file if it exists, in case
        // it was already solved or if we want to keep an older version
        write_toml(&slug, bite.libraries);
        write_markdown(&slug, bite.description, bite.author);
        write_exercise(bite.template);
        // TODO add error propagation ^^^

        // all done
        println!(" ✅");
    }
    Ok(())
}
