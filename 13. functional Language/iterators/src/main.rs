extern crate iterators;

use iterators::Counter;

fn main() {
    println!("Hello, world!");
    
    let sum : Vec<_> = Counter::new()
                            .zip(Counter::new().skip(1)) //1은 스킵.
                            .map(|(a,b)| a * b)
                            .filter(|x| x % 3 == 0)
                            .collect()
                            ;

    println!("{:?}", sum);
}