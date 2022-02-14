use draw_trait::{Screen, Pencil, Ballpen};

fn main() {
    println!("Hello, world!");

    let screen = Screen {
        components : vec![
            Box::new(Pencil {}),
            Box::new(Ballpen {}),
        ],
    };

    screen.run();
}
