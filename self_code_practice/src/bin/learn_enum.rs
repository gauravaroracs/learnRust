#[derive(Debug)]
enum Ipkind {
    V4 (String),
    V6 (String)
 }
 #[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main(){

    let home : Ipkind = Ipkind::V4((String :: from("127.0.0.1")));

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    let t1: (f32,f32) = (matrix.0,matrix.1);
    println!("{}", matrix);
    route(home);

}


fn route( x : Ipkind) {
    println!("Routing to {:?}",x);
}
