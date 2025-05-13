pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let mut result = add(2, 2);
        assert_eq!(result, 4);

        result = add(2, 3);
        assert_ne!(result, 0);
    }

    #[test]
    fn test_with_result() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // This test can be run using cargo test -- --ignored or cargo test -- --include-ignored
    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }

    #[test]
    fn destined_to_fail() {
        panic!("Make this test fail!")
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests_rectangle {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")

    // Uncomment this line to check detailed error message when test fails
    //format!("Hello!")
}

#[cfg(test)]
mod tests_greeting {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was {result}"
        );
    }
}

fn prints_and_returns_10(a: i32) -> i32 {
    // Run test as cargo test -- --show-output to show this line even for successes
    println!("I got the value {a}");
    10
}

#[cfg(test)]
mod tests_silly_function {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(value, 10);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(value, 5);
    }
}
