pub mod exam{
    pub mod exam_3{
        pub fn input_query() -> String{

            use std::io;
            println!("input query");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read query");

            let input = input.trim();
            if input.is_empty(){
                panic!("invalid query(doesn't support space only)");
            }
            input.to_string()
        }
    }
}