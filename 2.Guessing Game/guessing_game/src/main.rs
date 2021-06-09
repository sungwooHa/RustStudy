use std::io; //io standard library


fn main() 
{
    println!("Guess the number");
    println!("Please input your guess");

    //make variable 
    ////let 변수 선언
    ////mut : 가변 변수
    ////rust는 default로 불변이기 때문.
    //예시
    //// let foo = 5; //immutable
    //// let mut bar = 5; //mutable

    //가변 변수 생성
    let mut guess = String::new(); 

    //& : 참조자
    
    io::stdin().read_line(&mut guess).
    expect("failed to read line"); //Except

    println!("You guessed : {}", guess);
}
