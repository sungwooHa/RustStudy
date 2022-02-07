#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

// pub mod test_for_dll{
//     #[no_mangle]
//     pub extern fn greeting(){
//         println!("Hello World. this is rust");
//     }
// }

#[no_mangle]
pub extern "C" fn greeting(){
    println!("Hello World. this is rust");
}