use std :: collections::HashMap;


fn main(){

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);

    for (key,val) in &map{
      println!("{}-> {}",key,val);
    }

    println!("{},{}", field_name,field_value);
println!("{},{}", field_name,field_value);
    let temp = map.get(&field_name);
    println!("{:?}",temp)

}