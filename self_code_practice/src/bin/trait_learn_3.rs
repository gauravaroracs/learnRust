trait Area {
    fn area(&self) -> i32;
}

struct Square { side: i32 }
struct Rectangle { w: i32, h: i32 }

// TODO: implement Area for Square
impl Area for Square {

     fn area(&self) -> i32{
        self.side*self.side
     }
}

// TODO: implement Area for Rectangle
impl Area for Rectangle {
    fn area(&self) -> i32 {
        self.w*self.h
    }
    
}

fn print_area<T: Area>(shape: T) {
    println!("{}", shape.area());
}

fn main() {
    let s = Square { side: 4 };
    let r = Rectangle { w: 3, h: 5 };

    print_area(s);
    print_area(r);
}