
fn main() {
    let mut s = String::from("hello world");

    for(i, mut item) in s.iter_mut().enumerate(){//iter : 반복자, enumration() : 튜플로 반환 (index, &bytes[index])
        item = String::from("a");
    }


    let len = s.len();
    let slice = &s[3..s.len()];
    let slice = &s[3..]; //끝까지 일때 생략 가능
}
