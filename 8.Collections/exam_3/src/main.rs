mod lib;
use lib::exam::exam_3;
use std::collections::HashMap;

fn main() {

    println!("Type 'Add <name> to <department>' to add an employee");
    println!("Type 'List <department>' to list the employees of a department");
    println!("Type 'All' to list all employees by department");
    println!("Type 'Quit' to quit");

    let mut employee : HashMap<String, Vec<String>> = HashMap::new();

    loop{        
        let query = exam_3::get_query();
        match exam_3::Query::query_to_command(&query){
            Some(exam_3::Query::ADD{department, name}) => exam_3::execute_add(&department, &name, &mut employee) ,
            Some(exam_3::Query::LIST(department)) =>  exam_3::execute_list(&department, &employee),
            Some(exam_3::Query::ALL) => exam_3::execute_all(&employee),
            Some(exam_3::Query::QUIT) => break,

            None => println!("Input Error"),
        }
    }
}
