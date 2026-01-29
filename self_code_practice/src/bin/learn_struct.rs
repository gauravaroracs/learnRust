#[derive(Debug)]
struct Shape {
    length: u32,
    width: u32,
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl Shape {
    fn calculate_area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Shape) -> bool {
        other.length * other.width > self.calculate_area()
    }

    fn square(side: u32) -> Shape {
        Shape {
            length: side,
            width: side,
        }
    }
}

#[derive(Debug)]
struct Order {
    name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: u32,
}

fn create_order_template() -> Order {
    Order {
        name: String::from("Bob"),
        year: 2019,
        made_by_phone: false,
        made_by_mobile: false,
        made_by_email: true,
        item_number: 123,
        count: 0,
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn your_order() {
//         let order_template = create_order_template();

//         // TODO: Create your own order using the update syntax and template above!
//         let your_order = Order {
//             name: String::from("Hacker in Rust"),
//             count: 1,
//             ..order_template
//         };
//         assert_eq!(your_order.name, "Hacker in Rust");
//         assert_eq!(your_order.year, order_template.year);
//         assert_eq!(your_order.made_by_phone, order_template.made_by_phone);
//         assert_eq!(your_order.made_by_mobile, order_template.made_by_mobile);
//         assert_eq!(your_order.made_by_email, order_template.made_by_email);
//         assert_eq!(your_order.item_number, order_template.item_number);
//         assert_eq!(your_order.count, 1);
//     }
// }

// Structs contain data, but can also have logic. In this exercise, we have
// defined the `Package` struct, and we want to test some logic attached to it.

#[derive(Debug)]
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: u32,
}

impl Package {
    fn new(sender_country: String, recipient_country: String, weight_in_grams: u32) -> Self {
        if weight_in_grams < 10 {
            // This isn't how you should handle errors in Rust, but we will
            // learn about error handling later.
            panic!("Can't ship a package with weight below 10 grams");
        }

        Self {
            sender_country,
            recipient_country,
            weight_in_grams,
        }
    }

    // TODO: Add the correct return type to the function signature.
    fn is_international(&self) -> bool{
        // TODO: Read the tests that use this method to find out when a package
        // is considered international.
        if self.sender_country == self.recipient_country{
            return false;
        }
        else{
            return true;
        }
    }

    // TODO: Add the correct return type to the function signature.
    fn get_fees(&self, cents_per_gram: u32) -> u32{
        // TODO: Calculate the package's fees.
        self.weight_in_grams * cents_per_gram
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Austria");

        Package::new(sender_country, recipient_country, 5);
    }

    #[test]
    fn create_international_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Russia");

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(package.is_international());
    }

    #[test]
    fn create_local_package() {
        let sender_country = String::from("Canada");
        let recipient_country = sender_country.clone();

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(!package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Spain");

        let cents_per_gram = 3;

        let package = Package::new(sender_country, recipient_country, 1500);

        assert_eq!(package.get_fees(cents_per_gram), 4500);
        assert_eq!(package.get_fees(cents_per_gram * 2), 9000);
    }
}


fn main() {
    // let s: Point<f32> = Point {
    //     x :3.0,
    //     y:4.0
    // };

    // println!("{:?}", s.distance_from_origin());
    // // let rectangle_1: Shape = Shape {
    // //     length: 5,
    // //     width: 10,
    // // };
    // // let rectangle_2 = Shape {
    // //     length: 2,
    // //     width: 10,
    // // };
    // // let sq= Shape::square(5);
    // // let test_String = String::from("test_string");

    // // println!("dimensions of the shape are {:?}", sq);

    let order_template = create_order_template();
    println!("struct is {:?}", order_template);
}
