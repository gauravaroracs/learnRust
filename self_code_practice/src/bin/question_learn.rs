fn get_user_id(name: &str) -> Option<u32> {
    if name == "Bogdan" { Some(1) } else { None }
}

fn get_balance(id: u32) -> Option<f64> {
    if id == 1 { Some(100.50) } else { None }
}

// TODO: Complete this function using the '?' operator
fn get_user_balance(name: &str) -> Option<f64> {
    // 1. Get the ID. Use '?' so if it's None, this function returns None immediately.
    let id = get_user_id(name)?; 
    
    // 2. Get the balance using that ID.
    let balance = get_balance(id)?;

    
    // 3. Wrap the final result back in 'Some' because the return type is Option<f64>
    Some(balance)
}

// fn main() {
//     let balance = get_user_balance("Bogdan");
//     println!("Balance: {:?}", balance);
//     let v = vec![1, 2, 3];
// }
fn main() {
    let x: Option<i32> = Some(5);

    // TODO: make y = Some(10) using map
    let y: Option<i32> = x.map(|v| v +5);

    println!("{:?}", y);
}