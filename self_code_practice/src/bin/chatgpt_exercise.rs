#[derive(Debug)]
enum Node {
    Leaf(i32),
    Inner(Vec<i32>),
}

#[derive(Debug)]
struct Pair {
    a: i32,
    b: i32,
}

fn main() {
    let n1 = &Node::Leaf((5));
    let n2 = &Node::Inner((vec![1, 2, 3]));
    // let a: Option<i32> = None;
    let b: Option<i32> = Some((15));
    // let mut v = vec![Pair { a: 1, b: 2 }, Pair { a: 10, b: 20 }];
    // v[1].b +=5;
    let mut v = vec![1,2,3];
     v.push(4);
    let a = &v[0];
   
    println!("{}", a);
    // describe(n1);

    // describe_using_if_else(n1);
    // describe_using_if_else(n2);

    // println!("{}", sum_or_zero(n1));
    // println!("{}", sum_or_zero(n2));
    // println!("{:?}", add_optional(a, b));
    println!("{:?}", v);
}

fn describe(n: &Node) {
    match n {
        Node::Leaf(x) => println!("leaf value: {}", x),
        Node::Inner(items) => println!("inner has {} keys", items.len()),
    }
}

fn describe_using_if_else(n: &Node) {
    if let Node::Leaf((x)) = n {
        println!("leaf value: {}", x);
    } else if let Node::Inner((items)) = n {
        println!("inner has {} keys", items.len())
    }
}

fn sum_or_zero(n: &Node) -> i32 {
    match n {
        Node::Leaf(x) => *x,
        Node::Inner(items) => {
            let mut sum = 0;
            for i in items {
                sum += i;
            }
            sum
        }
    }
}

fn add_optional(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    match a {
        Some(x) => match b {
            Some(y) => Some((x + y)),
            None => None,
        },
        None => None,
    }
}
