
/// adds one to the number given.
/// # Example1
/// ### Example 1-1
/// ### Example 1-2
/// ```
/// assert_eq!(1, my_create::add_one(0));
/// assert_eq!(2, my_create::add_one(1));
/// ```
/// ```
/// code text
/// ```
/// normal text
pub fn add_one(x : i32) -> i32{
    x+1
}

/// adds two to the number given.
/// # Example
/// ```
/// assert_eq!(2, my_create::add_one(0));
/// assert_eq!(3, my_create::add_one(1));
/// ```
pub fn add_two(x : i32) -> i32{
    x+2
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        assert_eq!(add_one(1), 2);
    }
}
