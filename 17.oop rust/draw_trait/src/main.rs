use draw_trait::{Screen, SelectBox, Button, Screen_generic};

fn main() {
    println!("Hello, world!");

    let screen = Screen {
        components : vec![
            Box::new(SelectBox {
                width : 75,
                height : 10,
                options : vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width : 75,
                height : 10,
                label : String::from("Ok"),
            }),
        ],
    };

    screen.run();


    let screen_generic = Screen_generic{
        components : vec![
            SelectBox{
                width : 75,
                height : 10,
                options : vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            },
            SelectBox{
                width : 75,
                height : 10,
                options : vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            },
            SelectBox{
                width : 75,
                height : 10,
                options : vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            },
        ],
    };

    screen_generic.run();

}
