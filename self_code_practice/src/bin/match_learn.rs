enum Coin {
    Penny, 
    Nickel,
    Dime,
    Quater
    
}

fn main(){


    let coin : Coin = Coin::Dime;
    let test_var: Option<Option<i32>> = None;
    match test_var {
        Some(_) => println!("matched the some "),
        None => println!("matched the none part "),
    };



    let res= match coin{
        Coin::Penny => 1,
        Coin::Nickel => 2,
        Coin::Dime => 3,
        Coin::Quater => 5,
    };

    println!("The res is : {}", res);



}