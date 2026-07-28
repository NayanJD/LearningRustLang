use std::fmt::{Debug, Display};

use generics::{NewsArticleWithDefaultSummary, SocialPost, Summary, SummaryWithDefault};

struct Point<T, U> {
    x: T,
    // y: T  // This line wont allow line 64.
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

// To use references in a struct's fields, we need to specify lifetimes. This annotation
// 'a means an instance of ImportantExcerpt can’t outlive the reference it holds in its part field.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // First rule applies here and 'a is not required for the i32 return type
    fn level(&self) -> i32 {
        3
    }
}

// The lifetime parameter need to be declared here since we used a reference field
impl<'a> ImportantExcerpt<'a> {
    // Third rule applies here and the ouput &str is assigned the lifetime of
    // &self
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

fn main() {
    let list = vec![1, 2, 3, 4, 5];

    let largest = largest(&list);

    println!("Largest is {largest}");

    let wont_work = Point { x: 5, y: 4.0 };

    let tweet = SocialPost {
        username: String::from("nayandas"),
        content: String::from("It's too sunny today!"),
        reply: false,
        repost: false,
    };

    let article = NewsArticleWithDefaultSummary {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

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

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();

    let i = ImportantExcerpt {
        part: first_sentence,
    };
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

fn notify_multiple_traits(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item)
}

fn notify_multiple_trait_bounds<T: Summary + Display>(item: T) {
    println!("Breaking news! {}", item)
}

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    return 0;
}

fn some_function_concise<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    return 0;
}

fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("Horse Ebooks"),
        content: String::from("There was a black horse!"),
        reply: false,
        repost: false,
    }
}

// This does not work
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         SocialPost {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             repost: false,
//         }
//     }
// }

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

// This function does not work even after adding a lifetime 'a  because part is a reference. It's
// value is invalid after test function's scope. Even if we move i when we return the value, part
// still is a reference to a locally created value.
// fn test<'a>() -> ImportantExcerpt<'a> {
//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence = novel.split('.').next().unwrap();
//
//     let i = ImportantExcerpt {
//         part: first_sentence,
//     };
//
//     i
// }

// This is check whether the lifetime error occurs based on signature of the function.
// Looks like it does not!
// fn longest_static_str(x: &str, y: &str) -> &str {
//     "This"
// }

// This will fail because we are escaping a value which will go out of scope
// when returned from this function
// fn longest_with_escaping_value<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");

    if x.len() > y.len() {
        x
    } else {
        y
    }
}
