use generics::{Summary, Tweet};

struct Point<T, U> {
    x: T,
    // y: T  // This line wont allow line 26.
    y: U,
}

// Using T and U after impl is important if we want to use it in Point as in
// Point<T, U>
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<T1, U1>(self, point: Point<T1, U1>) -> Point<T, U1> {
        Point {
            x: self.x,
            y: point.y,
        }
    }
}

// Here we defined a function distance_from_origin which is only available for type
// Point<f32, f32>.
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let list = vec![1, 2, 3, 4, 5];

    let largest = largest(&list);

    println!("Largest is {largest}");

    let wont_work = Point { x: 5, y: 4.0 };

    let tweet = Tweet {
        username: String::from("nayandas"),
        content: String::from("It's too sunny today!"),
        reply: false,
        retweet: false,
    };

    // importing Summary trait is important to use summarize() on the type
    // Tweet
    println!("1 new tweet: {}", tweet.summarize());

    let r;

    {
        let x = 5;
        r = &x;
    }
    // x dropped here hence r cannot be used
    //
    // println!("r: {r}")

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");

        let result = longest(&string1, &string2);

        println!("The longer string is {result}")
    }

    // This line is not valid because in the definition of longest function the returned value has
    // a lifetime of smaller among the provided arguments. Here, string2 has smaller lifetime and
    // hence result also has the same lifetime as string2.
    //
    // println!("The longer string is {result}")
}

fn largest(list: &[i32]) -> &i32 {
    let mut largest: &i32 = &list[0];

    for val in list {
        if val > largest {
            largest = val;
        }
    }

    return largest;
}

fn largest_gen<T>(list: &[T]) -> &T {
    let mut largest: &T = &list[0];

    for val in list {
        if val > largest {
            largest = val;
        }
    }

    return largest;
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

fn notify_traitbounds<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize())
}

// The section https://doc.rust-lang.org/book/ch10-02-traits.html#specifying-multiple-trait-bounds-with-the--syntax
// is not put into code here.

// This function wont work because compiler does not know which of x or y is being returned. This
// does not pass borrow checker until lifetime is defined
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
