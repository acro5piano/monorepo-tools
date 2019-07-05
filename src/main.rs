use serde_json::{Result, Value};
use std::fs;

fn main() {
    let contents = fs::read_to_string("./package.json")
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);

    // untyped_example();

    let v: Value = serde_json::from_str(contents);

    // let v: Value = parse(contents);

    // println!("scripts: \n{}", v["scripts"]);
}

fn parse(contents: String) -> Result<()> {
    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(contents)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}

fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}
