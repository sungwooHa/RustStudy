pub trait Draw{
    fn draw(&self);
}

pub struct Pencil {}
pub struct Ballpen {}

impl Draw for Pencil{
    fn draw(&self) {
        println!("pencil draw");
    }
}

impl Draw for Ballpen{
    fn draw(&self){
        println!("ballpen draw");
    }
}

pub struct Screen{
    pub components : Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter(){
            component.draw();
        }
    }
}
