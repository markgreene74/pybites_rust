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

fn write_root_toml(base_path: &Path) {
    dbg!(base_path.join("Cargo.toml"));
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
    // define the base_path (current directory / exercises)
    let base_path = env::current_dir().unwrap().join("exercises");

    print!("Downloading the exercises from Pybites Rust (rustplatform.com)");
    let client = reqwest::blocking::Client::new();
    let response = client.get("https://rustplatform.com/api/").send()?;
    println!(" ✅");
    println!(
        "'exercises' will be created in the current directory ({})",
        base_path.display()
    );

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

    // make sure the base path (exercises) exists
    fs::create_dir_all(&base_path)?;

    for bite in bites {
        print!("{:#?}", bite.name);
        let slug = bite.slug;
        let exercise_path = &base_path.join(&bite.level).join(&slug);

        // make sure the exercise directory exists
        fs::create_dir_all(&exercise_path)?;
        // re-write/update the toml and md files but make  a backup copy of the
        // exercise file if it exists, in case it was already solved
        write_toml(&slug, bite.libraries);
        write_markdown(&slug, bite.description, bite.author);
        write_exercise(bite.template);
        // TODO add error propagation ^^^
        println!(" ✅");
    }
    write_root_toml(&base_path);
    // TODO add error propagation ^^^

    // all done
    Ok(())
}
