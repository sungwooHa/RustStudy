pub trait Draw{
    fn draw(&self);
}
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button Draw");
    }
}
pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox Draw");
    }
}

//trait bound
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

//Generic type parameter
pub struct Screen_generic<T: Draw> {
    pub components : Vec<T>,
}

impl<T> Screen_generic<T> where T: Draw{
    pub fn run(&self) {
        for component in self.components.iter(){
            component.draw();
        }
    }
}

