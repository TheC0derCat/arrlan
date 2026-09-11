mod matrix;
use matrix::*;

fn main() {
    println!("Hello, world!");
    let a: Matrix = Matrix::new(vec![5, 5, 3]);
    let b: Matrix = Matrix::new(vec![2, 2, 2]);
    let c: Matrix = a + b;
    println!("{:?}", c);
}
