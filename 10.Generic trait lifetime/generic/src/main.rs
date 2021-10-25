
struct Point<T,U>{
    x : T,
    y: U,
}

impl<T, U> Point<T, U>{
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    fn mixup<V, W>(self, other:Point<V,W>) -> Point<T,W>{
        Point {
            x : self.x,
            y : other.y,
        }
    }
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.5, y: 4.5 };
    let integer_and_float = Point { x: 5, y: 4.5 };

    println!("x {}, y {}", both_integer.x(), both_integer.y());
    println!("x {}, y {}", both_float.x, both_float.y);
    println!("x {}, y {}", integer_and_float.x, integer_and_float.y);

    println!("mix up, this.x, other.y");
    let mixed_point = both_integer.mixup(both_float);
    println!("x {}, y {}", mixed_point.x, mixed_point.y);
}
