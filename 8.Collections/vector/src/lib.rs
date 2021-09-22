
pub mod collection{
    pub mod vector_practice{
        fn create_i32(range_start:i32, range_end:i32) -> Vec<i32>{
            let mut v : Vec<i32> = Vec::new();
            for value in range_start..range_end{
                v.push(value);
            }
            v
        }
        
        fn print_i32(v : &Vec<i32>){
            for value in v{
                println!("value : {}", value);
            }

            let mid_index = {
                v.len() >> 1
            };

            let mid_value : &i32 = &v[mid_index];

            println!("mid value : {}, mid index : {}", mid_value, mid_index);
        }

        fn plus_i32(plus_value : i32, v : &mut Vec<i32>){
            for value in v {
                *value += plus_value;
            }
        }

        pub fn excute_i32(){
            let range_start = 10;
            let range_end = 20;
            let v = create_i32(range_start, range_end);
            print_i32(&v);
        
            let mut v_copy = v;
            plus_i32(30, &mut v_copy);
            print_i32(&v_copy);
        }

        pub fn excute_enum(){
            #[derive(Debug)]
            enum SpreadSheetCell{
                Int(i32),
                Float(f64),
                Text(String),
            }
        
            let row = vec![
                SpreadSheetCell::Int(3),
                SpreadSheetCell::Text(String::from("blue")),
                SpreadSheetCell::Float(10.12),
            ];
        
            for enum_value in row{
                println!(" enum val : {:?}", enum_value);
            }
        }
    }
}

