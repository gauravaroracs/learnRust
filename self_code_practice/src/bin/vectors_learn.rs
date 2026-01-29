#[derive(Debug)]
enum Vehicle {

    Size (i32),
    Colour (String)
    
}

fn main() {

    let mut v: Vec<Vehicle>= vec![Vehicle::Size(1),Vehicle::Colour("red".to_string()),Vehicle::Size(2),Vehicle::Colour("green".to_string())];

    // for i in &mut v {
    //     *i*=10;
    // }
    
    for i in &v {
        println!("{:?}",i);
    }
    // sorted_insert(&mut v, 5);
    // // for i in v {
    // //     println!("{i}");
    // // }
    // // v should become [1, 3, 4, 5, 10]
    // let res = match  find_in_vec(&v, 5) {
    //     Some(x) => println!("found the element at {}", x),
    //     None => println!("Element not found"),
    // };
}

fn sorted_insert(v: &mut Vec<i32>, x: i32) {
    let mut temp: Vec<i32> = Vec::new();
    let mut pos= 0;

    for (i, val) in v.iter().enumerate() {
        if *val >= x {
            pos = i;
            break;
        } 
    }
    if pos == 0{
        pos = v.len();
    }
    
    safe_insert(v,pos, x);
    
}


fn safe_insert<T>(v: &mut Vec<T>, index: usize, value: T) {
    if index <= v.len() {
        v.insert(index, value);
    } else {
        println!("Invalid index!");
    }
}


fn find_in_vec(v: &[i32], x: i32) -> Option<usize> {
    for (i, val) in v.iter().enumerate() {
        if *val == x { 
            return Some(i);
        } 
    }
    None
}
