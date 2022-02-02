pub mod word_data_base{
    pub mod word_manager{
        //use scraper::{Html, Selector};
        use std::io::{stdout, Write};
        use curl::easy::Easy;
        //use rand::Rng;

        const WORD_SITE : &'static str = "https://www.bestwordlist.com/5letterwords.htm";

        pub fn word_getter() /*-> Option<Vec<string>>*/{
            let mut easy = Easy::new();
            easy.url(WORD_SITE).unwrap();
            easy.write_function(|data| {
                stdout().write_all(data).unwrap();
                Ok(data.len())
            }).unwrap();
            easy.perform().unwrap();
        
            println!("{}", easy.response_code().unwrap());
        }
    }
}