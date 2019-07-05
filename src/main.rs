use serde_json::{Value};
use std::fs;
use std::io;

fn main() {
    let contents = fs::read_to_string("./package.json")
        .expect("Something went wrong reading the file");

    let v = parse(&contents);

    let v = match v {
        Ok(strings) => strings,
        Err(_e) => panic!("No script"),
    };

    println!("scripts: \n{}", v[0]);

    // untyped_example();
    // let v: Value = parse(contents);
    // println!("scripts: \n{}", v["scripts"]);
}

fn parse(contents: &str) -> Result<Vec<String>, io::Error> {
    let v: Value = serde_json::from_str(contents)?;

    Ok(vec![v["scripts"]["test"].to_string()])
}
