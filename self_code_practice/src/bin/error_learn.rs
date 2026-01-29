use std::error::Error;

fn get_double_from_file() -> Result<i32, Box<dyn Error>> {
    // TODO: read number from "num.txt"
    let text = std::fs::read_to_string("src/bin/num.txt")?;

    // TODO: parse i32
    let n = text.trim().parse::<i32>()?;

    Ok(n * 2)
}

fn main() {
    match get_double_from_file() {
        Ok(v) => println!("{}", v),
        Err(e) => println!("Error: {}", e),
    }
}