pub mod exam{
    pub mod exam_1{
        use std::io;

        pub fn input_string() -> String{

            println!("input new string  for Pig Latin");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read typing");

            println!("previous input :  '{}',  length : {}", input, input.len());

            let input = input.trim();
            if input.is_empty(){
                panic!("invalid typing(doesn't support space only)");
            }
            input.to_string()
        }

        pub fn convert_to_pig_latin(input : String) -> String{

            let (first, remains) = split_first_char(input);

            println!("first : {}, remains : {}" , first, remains);

            match is_vowel(&first.chars().next().unwrap()){
                true => format!("{}-{}hay", remains, first),
                false => format!("{}-{}ay", remains, first),
            }
        }

        fn split_first_char(s:String) -> (String, String){
            
            let mut s = s.clone();
            let remains = s.split_off(1);

            (s, remains)
        }

        fn is_vowel(ch : &char) -> bool
        {
            let low_ch = ch.to_lowercase().to_string().chars().next().unwrap();
            match low_ch {
                'a'|'e'|'i'|'o'|'u' => true,
                _ => false,
            }
        }

    }
}