// TODO: Fix the compiler error in the function without adding any new line.
// This function borrows the vector mutably and edits it in place; no ownership is moved out.
fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(88);

    () // Explicit unit return (same as just ending the function).
}

// Shouldn't take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}


fn main() {
    // 1st concept
    // let mut vec1= vec![1,2,3];
    // // Mutably borrow vec1 to change it, then take an immutable borrow for reading.
    // fill_vec(&mut vec1);
    // let vec2 = &vec1;

    // // Only one mutable borrow at a time; here we borrow vec2 mutably inside fill_vec,
    // // then use both values after the borrow ends.
    // println!(" vec 1 {:?}, vec 2 {:?}",vec1,vec2);

    //2nd concept
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);


}
