#![allow(unused)]
fn main() {
'outer: loop {
    while true {
        break 'outer;
    }
}
}