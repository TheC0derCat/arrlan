mod matrix;
use matrix::*;

fn main() {
    println!("Hello, world!");
    let a: Matrix = Matrix::new(vec![5, 5, 3], vec![3]).unwrap();
    let b: Matrix = Matrix::new(vec![2, 2, 2], vec![3]).unwrap();
    let c: Matrix = a - b;
    println!("{:?}", c);
    println!("sum: {}", c.sum());
    println!("product: {}", c.product());
    let c: Matrix = c.sort();
    println!("{:?}", c);
}
