pub mod exam{
    pub mod exam_3{
        use std::collections::HashMap;
        use std::io;

        pub enum Query{
            ADD{department :  String, name : String},
            LIST(String),
            ALL,
            QUIT,
        }

        impl Query
        {
            pub fn query_to_command(str_query : &Vec<String>) -> Option<Self>{
                
                let str_query : Vec<&str>  = str_query.iter().map(|s|s.as_ref()).collect();

                match str_query.as_slice() {
                    ["All"] => Some(Query::ALL),
                    ["Quit"] => Some(Query::QUIT),
                    ["List", department] => Some(Query::LIST(department.to_string())),
                    ["Add", name, "to", department] => Some(Query::ADD{
                        name :  name.to_string(),
                        department : department.to_string()
                    }),
                    _ => None,
                }
            }
        }

        pub fn get_query() -> Vec<String>{
            println!("input query");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read query");
            let input = input.trim();
            if input.is_empty(){
                panic!("invalid query(doesn't support space only)");
            }
            
            let mut output_string:Vec<String> = Vec::new();
            let words : Vec<&str> = input.split_whitespace().collect();
            for word in words {
                output_string.push(String::from(word));
            }
            output_string
        }
        
        pub fn execute_add(department : &String, name : &String, employee : &mut HashMap<String, Vec<String>>){
             employee.entry(department.to_string()).or_default().push(name.to_string());
        }
        pub fn execute_list(department : &String, employee : &HashMap<String, Vec<String>>){
            match employee.get(department){
                Some(names) => {
                    for name in names {
                        println!("{} : {}", department, name);
                    }
                },
                None=> println!("can't find that department"),
            }
        }
        pub fn execute_all(employee : &HashMap<String, Vec<String>>){
            for (department, names) in employee {
                let mut names = names.clone();
                names.sort();
                for name in names {
                    println!("{} : {} ", department, name);
                }
            }
        }
    }
}