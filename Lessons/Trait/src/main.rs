use std::ops::Mul;

trait Shape<T> {
    fn area(&self) -> T;
}

struct Rectangle<T: Mul> {
    x: T,
    y: T
}

impl <T> Shape<T> for Rectangle<T>
    where T: Mul<Output = T> + Copy {
        fn area(&self) -> T {
            self.x * self.y
        }
}

fn main() {
    let rectangle1 = Rectangle { x: 3, y: 7 };
    let rectangle2 = Rectangle { x: 5.3, y: 2.6 };
    
    println!("{}", rectangle1.area());
    println!("{}", rectangle2.area());
}
