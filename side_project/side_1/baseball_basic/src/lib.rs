
pub struct Game{

    word : String,
    game_log : Vec<log>,
    max_round : u32,

    enum Log{
        Strike(u32),
        Ball(u32),
        Guess(String),
    }

    impl Log{

        pub fn new(Strkie : u32, Ball : u32, Guess : &String) -> Log {
            Log {
                Strike,
                Ball,
                Guess : Guess.clone(),
            }
        }

        pub fn print_log(){
            println!("Guess : {}, Strike : {}, Ball : {}", &Guess, Strike, Ball);
        }
    }    
}

impl Game{
    pub fn new(word : String, round : u32) -> Game{
        Game{
            word,
            max_round = u32,
        }
    }

    pub fn Try(&mut self, guess_word : &String) -> Result<bool, &str>{
        //strike, ball
        //log ++
        if(!Verify())
            panic!("wrong word, different length");

            
        let count_strike = match CountStrike(guess_word){
            Some(v) =>{
                if v == word.len(){
                    //game end, success
                }
                else 
                    v
            }
            None => 0,
        };

        let count_ball = match CountBall(guess_word){
            Some(v) => v,
            None => 0,
        };

        let log = Log::new(count_strike, count_ball, guess_word));
        log.print_log();
        self.game_log.push(log);

        self.game_log.len >= max_round ? {
            //fail
        } : {
            //keep
        }
    }

    fn Verify(&self.word, guess_word : &String) -> bool{
        
        return self.word.len() == guess_word.len()
    }

    fn CountStrike(&self.word, guess_word : &String) -> Option<u32>{

    }

    fn CountBall(&self.word, guess_word : &String) -> Option<u32>{

    }

    pub fn GetAnswer(self) -> &str{
        return &word;
    }
}


pub mod word{

    fn get_word_pool() -> Result<String> {
        
    }
    pub mod generator{
        
    }
}