pub mod exam{
    pub mod exam_1{
        use rand::Rng;
        use std::collections::HashMap;

        pub fn get_temp_vec(size : usize, range : std::ops::Range<i32>) -> Vec<i32>{

            let mut v : Vec<i32> = Vec::with_capacity(size);
            let mut rng = rand::thread_rng();
            
            for _index in 0..size {
                v.push(rng.gen_range(range.start..range.end));
            }

            println!("{:?}", v);
            v
        }

        pub fn get_min(v : &Vec<i32>) -> Option<&i32>{
            
            match v.iter().min(){
                Some(min) => Some(min),
                None => {
                    println!("empty vector");
                    None
                }
            }
        }

        pub fn get_max(v : &Vec<i32>) -> Option<&i32>{
            match v.iter().max(){
                Some(max) => Some(max),
                None => {
                    println!("empty vector");
                    None
                }
            }
        }

        pub fn get_mode(v : &Vec<i32>) -> Option<i32>{
            if v.len() == 0 {
                return None;
            }
            let mut map = HashMap::new();

            for val in v {
                *map.entry(val).or_insert(0) += 1;
            }
            println!("{:?}", map);

            let top_val = **map.iter().max_by(|a,b| a.1.cmp(&b.1)).unwrap().0;
            Some(top_val)
        }

        pub fn get_average(v : &Vec<i32>) -> Option<i32>{
            if v.len() == 0{
                return None;
            }

            let mut sum = 0;
            for &val in v{
                sum += val;
            }

            println!("sum : {}, size : {}", sum, v.len());

            Some(sum / v.len() as i32)
        }

        pub fn get_median(v : &Vec<i32>) -> Option<i32>{
            if v.len() == 0{
                return None;
            }

            let mut v = v.to_vec();
            v.sort();

            println!("{:?}", v);

            Some(v[(0 + v.len() - 1)/2])
        }
    }
}