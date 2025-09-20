// Download the exercises from https://rustplatform.com/
// and make them available locally.

use anyhow::Result;
use std::env;

fn main() -> Result<()> {
    println!("Download the exercises from Pybites Rust");

    // collect arguments
    let args: Vec<String> = env::args().collect();

    // send the request
    let response = reqwest::blocking::get("https://rustplatform.com/api/")?;

    // just testing, print out the status and headers and exit
    if args.contains(&String::from("--test")) {
        println!("Status: {}", response.status());
        println!("Headers:\n{:#?}", response.headers());
        return Ok(());
    }
    // println!("Body:\n{}", body);
    println!("Status: {}", response.status());
    Ok(())
}
