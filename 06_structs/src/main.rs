struct User {
    active: bool,
    username: String, // Cant use &str here since references in struct requires defineing lifetimes
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn width(&self) -> bool {
        return self.width > 0;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.height >= other.height && self.width >= other.width;
    }

    fn square(width: u32) -> Self {
        Self {
            width,
            height: width,
        }
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("nayan1996"),
        email: String::from("dastms@gmail.com"),
        sign_in_count: 0,
    };

    user1.sign_in_count = user1.sign_in_count + 1;

    let mut user2 = build_user(String::from("dastms1@gmail.com"), String::from("nayan"));

    // struct update syntax to create another concrete value from existing concrete value.
    // The ..user2 must come last. Also, user1.username has been moved to user2.username. Hence,
    // We cannot use user1 as whole. However, user1.email is valid. Check line  39
    let mut user3 = User {
        email: String::from("dastms2@gmail.com"),
        ..user2
    };

    // let email = user2.username;

    // Each struct you define is its own type, even though the fields within the struct might have the same types.
    // That is, a function that takes a parameter of type Color cannot take a Point as an argument,
    // even though both types are made up of three i32 values.
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Unlike tuples, tuple structs require you to name the type of the struct when you destructure them.
    let Color(x, y, z) = black;

    let subject = AlwaysEqual;

    let width1 = 30;
    let height1 = 60;

    println!("The area of the rectangle is {}.", area(width1, height1));

    let scale = 2;
    let rectangle1 = Rectangle {
        width: dbg!(scale * 30), // dbg! returns ownership of value to width
        height: 60,
    };

    // println!(
    //     "The area of rectangle {:#?} using struct is: {}",
    //     rectangle1,
    //     area_rectangle(&rectangle1)
    // );

    // dbg! takes ownership of the passed arguments and returns it. It won't work without
    // #[derive(Debug)] annotation
    dbg!(&rectangle1);

    println!("The area of rectange1 is {}!", rectangle1.area());

    if rectangle1.width() {
        println!("The rectangle has non-zero width.")
    }

    println!(
        "Can hold: {}",
        rectangle1.can_hold(&Rectangle {
            width: 0,
            height: 0
        })
    );

    println! {"This is a square {:?}", Rectangle::square(20)}
}

fn build_user(email: String, username: String) -> User {
    // This uses field init shorthand to initialise struct values
    User {
        active: true,
        username,
        email,
        sign_in_count: 0,
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_rectangle(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
