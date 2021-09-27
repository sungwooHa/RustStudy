pub mod example{
    #[allow(dead_code)]
    pub mod collection{
        pub mod vector{
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

            pub fn execute_i32(){
                let range_start = 10;
                let range_end = 20;
                let v = create_i32(range_start, range_end);
                print_i32(&v);
            
                let mut v_copy = v;
                plus_i32(30, &mut v_copy);
                print_i32(&v_copy);
            }

            pub fn execute_enum(){
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

        pub mod hash{
            use std::collections::HashMap;

            fn print_hash_test<T : std::fmt::Display, Y : std::fmt::Display>(h : &HashMap<T, Y>){
                for (key, value) in h{
                    println!("key : {}, value : {}", key, value);
                }
            }            

            pub fn execute_8_20(){
                let mut scores = HashMap::new();
                
                scores.insert(String::from("Blue"), 10);
                scores.insert(String::from("Red"), 50);

                print_hash_test(&scores);

            }

            pub fn execute_8_21(){
                let teams = vec![String::from("Blue"), String::from("Yellow")];
                let initial_scores = vec![10,50];

                let scores: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();

                print_hash_test(&scores);
            }
            
            pub fn execute_ownership(){
                let field_name = String::from("Favorite color");
                let field_value = String::from("Blue");

                let mut map = HashMap::new();
                map.insert(&field_name, &field_value);

                print_hash_test(&map);
                println!("{}", field_name);       //valid
                println!("{}", field_value);      //valid

                let mut map = HashMap::new();
                map.insert(field_name, field_value);
                print_hash_test(&map);

                //println!("{}", field_name);       //invalid
                //println!("{}", field_value);      //invalid

            }

            pub fn execute_access_value(){
                let mut scores = HashMap::new();

                scores.insert(String::from("Blue"), 10);
                scores.insert(String::from("Yellow"), 50);
                            
                let team_name = String::from("Blue");
                let score = scores.get(&team_name); 
                println!("score : {:?}", score);
            }

            pub fn execute_update(){
                let mut scores = HashMap::new();
                scores.insert(String::from("Blue"), 10);

                scores.entry(String::from("Yellow")).or_insert(50);
                scores.entry(String::from("Blue")).or_insert(50); //없으면값을 넣는다.
                println!("score : {:?}", scores);

            }
        }
    }
}
