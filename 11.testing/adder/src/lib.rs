#[cfg(test)]
mod tests {
    // #[test]
    // fn exploration() {
    //     assert_eq!(2 + 2, 4);
    // }

    
    // pub fn greeting(name: &str) -> String {
    //     format!("Hello! {}!", name)
    //     //String::from("Hello")
    // }

    // #[test]
    // fn greeting_contains_name() {
    //     let result = greeting("Carol");
    //     assert!(
    //         result.contains("Carol"),
    //         "Greeting did not contain name, value was `{}`", result
    //     );
    // }

    
    pub struct Guess {
        value: u32,
    }

    impl Guess {
        pub fn new(value: u32) -> Guess {
            if value < 1 {
                panic!("Guess value must be less than or equal to 100, got {}.", value);
            } else if value > 100 {
                panic!("Guess value must be greater than or equal to 1, got {}.", value);
            }
            Guess {
                value
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        #[should_panic(expected = "Guess value must be less than or equal to 100")]
        fn greater_than_100() {
            Guess::new(200);
        }
    }
}
