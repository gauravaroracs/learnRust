use std::array;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Rectangle {
    width: f32,
    height: f32,
}
impl Rectangle {
    pub fn area(&self) -> f32 {
        self.width * self.height
    }
}

// fn main() {
//     let array = [1; 5];
//     // println!("Array {:?}",&array[1..7]);
//     let mut counter = 0;
//     let result = loop {
//         counter += 1;
//         if counter == 10 {
//             break counter;
//         }
//     };
//     // println!("Res is {}",result);
//     // let mut optional = Some(0);
//     // while let Some(i) = optional {
//     //     print!("{}", i);
//     // }

//     let mut s2 = String::from("hello");
//     // let s2_ref = &mut s2; // mutable reference
//     s2.push_str(" world!");
//     // println!("string is. {}",s2);
//     let p = Point { x: 102, y: 0 };

//     //   match p {
//     //       Point { x, y: 0 } => println!("{}", x),
//     //       Point { x, y } => println!("{} {}", x, y),
//     //   }

//     let user = User { id: 120 };

//     match user {
//         User {
//             id: id_variable @ 3..=7,
//         } => println!("id: {}", id_variable),
//         User { id: 10..=12 } => println!("within range"),
//         User { id } => println!("id: {}", id),
//     }

//     let some_string = Some("LGR".to_owned());
//     let some_len = some_string.map(|s| s.len());
//     println!("len is {:?}",some_len.unwrap());

    
// }

// fn main() {
//     let numbers = vec![1, 2, 3];
    
//     // TODO: Use .iter(), .map(), and .collect() to create 'doubled'
//     // Hint: .map(|x| ... )
//     let doubled: Vec<i32> = numbers.iter()
//                             .map(|x| x*2)
//                             .collect();


//     println!("{:?}", doubled); // Should print [2, 4, 6]
// }

fn main() {
    // let numbers = vec![1, 2, 3, 4, 5, 6];

    // // TODO: Keep only even numbers (x % 2 == 0)
    // let evens: Vec<i32> = numbers.into_iter()
    // .filter(|x| x % 2 == 0)
    // .collect();

    // println!("{:?}", evens); // Should print [2, 4, 6]


    //  let data = vec!["1", "x", "3", "y"];

    // let (oks, errs): (Vec<_>, Vec<_>) = data
    //     .into_iter()
    //     .map(|s| s.parse::<i32>())
    //     .partition(Result::is_ok);

    // let numbers: Vec<i32> = oks.into_iter().map(Result::unwrap).collect();
    // let errors: Vec<_> = errs.into_iter().map(Result::unwrap_err).collect();

    // println!("{:?}", numbers);
    // println!("{:?}", errors);

    // fn largest<T: &Vec>(list: &[T]) -> T {
    // let mut max = list[0];

    // for &item in list {
    //     if item > max {
    //         max = item;
    //     }
    // }

    // max


    {

    }
    let a = vec![3, 7, 2];
    let b: Vec<f64> = vec![1.2, 9.4, 3.1];

    // TODO: call largest for both
    // let x = largest(&a);
    // let y = largest(&b);

    // println!("{}", x);
    // println!("{}", y);
   let res;
{
    let a = "abcd";
    let b = "ab";
    res = first(&a, &b);
}
println!("{}", res); // error: res would point to dropped Strings
    

 let x = Rc::new(RefCell::new(10));

    let a = Rc::new(&x);
    let b = Rc::new(&x);

    *a.borrow_mut() += 5;
    *b.borrow_mut() += 5;

    println!("{}", x.borrow());

    let my_closure = |x| vec![x*2];
assert_eq!(my_closure(5), vec![10]);

}

fn first<'a>(a: &'a str, b: &str) -> &'a str {
    a
}
// fn parse_all(strings: Vec<&str>) -> Result<Vec<i32>, ParseIntError> {
//     let mut numbers = Vec::new();

//     for s in strings {
//         // .parse returns a Result. 
//         // We use '?' to either get the 'i32' or exit the function with an error.
//         let n = s.parse::<i32>()?; 
//         numbers.push(n);
//     }

//     Ok(numbers)
// }

struct Point {
    x: i32,
    y: i32,
}

struct User {
    id: i32,
}
