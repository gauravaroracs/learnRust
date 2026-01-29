fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        let result = numerator / denominator;
        Some(result)
    }
}

fn main() {
    let outcome = divide(10.0, 2.0);
    
    match outcome {
        Some(val) => println!("Result: {}", val),
        None => println!("Cannot divide by zero!"),
    }
}