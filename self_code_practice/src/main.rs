// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;

mod learn_struct;

fn main() {
    //Making a guessing game

    // const SECONDS_IN_A_HOUR : u32 = 60 * 60;

    // let secret_number = rand::rng().random_range(1..=100);
    // println!("Hello, welcome to the game of guess the number!");
    // loop {
    //     let mut guess = String::new();

    //     println!("Plese input your number: ");
    //     io::stdin().read_line(&mut guess).expect("invalid input");
    //     let guess: i32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => {
    //             println!("Please enter a valid number!");
    //             continue;
    //         }

    //     };

    //     match guess.cmp(&secret_number) {
    //         Ordering::Equal => {
    //             println!("You guessed it right!");
    //             break;
    //         }
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //     }

    // }

    //checking the shadowing part
    // let mut t= "test string";
    // println!("initial value of t {}", t);
    // {
    //     let t= 5;
    //     println!("value of t in new scope:{} ", t);

    // }

    // println!("value of t in end {}", t);

    //checking how to convert from one data type to another

    // let a= "56";

    // let b: u32 = a.parse().expect("invalid number");

    // println!("new values is {}",b);

    //LEARNING ABOUT ARRAYS

    // let arr = [1, 2, 3, 4, 5];
    // let mut t  = String::new();

    // io::stdin().read_line(&mut t).expect("failed to read line");

    // let t= t.trim().parse::<usize>().expect("invalid input");

    // println!("value of t is {}", t);

    // println!("value of arr is {}", arr[t ]);

    //LEARNING ABOUT FUNCTIONS

    // let mut test_string = String::from("hello ");

    // let r1 = &mut test_string;
    // r1.push_str("world ");
    // let r2 = &mut test_string;
    // let length = calculate_len(r2);
    // println!("The length of '{}' is {}.", test_string, length);

    let test_string = String::from("helloworld you are the best");
    println!("{}", first_word(&test_string));
}

fn first_word(input: &str) -> &str {
    let s = input.as_bytes();
    for (i, &item) in s.iter().enumerate() {
        if item == b' ' {
            return &input[..i];
        }
        
    }
    &input[..]
}
