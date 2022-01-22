fn main() {

    #[derive(Debug)]
    enum IpAddr{
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));
    dbg!(&home);

    println!("v4 : {:?}, v6 : {:?}", home, loopback);

    #[derive(Debug)]
    struct IpAddrStruct{
        v4 : (u8,u8,u8,u8),
        v6 : String,
    }

    let str_ipaddr = IpAddrStruct {
        v4 : (127,0,0,1),
        v6 : String::from("::1"),
    };

    dbg!(&str_ipaddr);
    println!("Hello, world!");

    // 열거를 활용한 방법
    #[derive(Debug)]
enum Message{
	Quit, //연관 데이터 없음
	Move{x:i32, y:i32}, //익명 구조체
	Write(String), //String 포함
	ChangeColor(i32,i32,i32), //3개의 i32 -> 튜플구조체야?
}
    let x = 10;
    let y = 5;
    let move_message = Message::Move{x,y};
    dbg!(&move_message);

    let change_color_message = Message::ChangeColor(1,2,3);
    dbg!(&change_color_message);


    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin : Coin) -> u32{
        match coin{
            Coin::Penny => {
                println!("Lucky penny!");
                return 1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    value_in_cents(Coin::Penny);
}
