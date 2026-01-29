

fn main(){

    let a = String::from("hello");
    let res ;
    
    let b = String::from("world123");
   res=longest(&a,&b);
    

    println!("The longest string is {}",res);
}

fn longest<'a> (x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}