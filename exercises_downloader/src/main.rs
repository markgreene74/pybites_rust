// Download the exercises from https://rustplatform.com/
// and make them available locally.

// use reqwest::Error;  // DEBUG
use serde::Deserialize;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
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

fn write_root_toml(path: &Path) -> std::io::Result<()> {
    // main Cargo.toml template
    let content = b"[package]
name = \"exercises\"
version = \"0.1.0\"
edition = \"2024\"\
\n
[workspace]
resolver = \"3\"
members = [\"adder\"]";

    let filename = path.join("Cargo.toml");
    let mut file = File::create(filename)?;
    file.write_all(content)?;
    Ok(())
}

fn write_toml(path: &Path, slug: &String, libraries: String) -> std::io::Result<()> {
    // exercise Cargo.toml template
    let content = "[package]
name = \"package_name\"
version = \"0.1.0\"
edition = \"2024\"
[dependencies]\n"
        .replace("package_name", slug.as_str());

    let filename = path.join("Cargo.toml");
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;
    file.write_all(libraries.as_bytes())?;
    Ok(())
}

fn write_exercise(path: &Path, template: String) -> std::io::Result<()> {
    let src_dir = path.join("src");
    fs::create_dir_all(&src_dir)?;
    let filename = src_dir.join("lib.rs");
    let mut file = File::create(filename)?;
    file.write_all(template.as_bytes())?;
    Ok(())
}

fn write_markdown(
    path: &Path,
    slug: &String,
    description: String,
    author: String,
) -> std::io::Result<()> {
    dbg!(&path);
    dbg!(&slug);
    dbg!(&description);
    dbg!(&author);
    Ok(())
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
        // re-write/update the toml and md files but make a backup copy of the
        // exercise file if it exists, in case it was already solved
        write_toml(&exercise_path, &slug, bite.libraries)?;
        write_markdown(&exercise_path, &slug, bite.description, bite.author)?;
        write_exercise(&exercise_path, bite.template)?;
        println!(" ✅");
    }

    write_root_toml(&base_path)?;

    // all done
    Ok(())
}
