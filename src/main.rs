use serde_json::{Value};
use std::fs;
use std::io;

fn main() {
    let contents = fs::read_to_string("./package.json")
        .expect("Something went wrong reading the file");

    let v = match parse(&contents) {
        Ok(strings) => strings,
        Err(_e) => panic!("No script"),
    };

    println!("scripts: \n{}", v[0]);
}

fn parse(contents: &str) -> Result<Vec<String>, io::Error> {
    let v: Value = serde_json::from_str(contents)?;

    Ok(vec![v["scripts"]["test"].to_string()])
}
