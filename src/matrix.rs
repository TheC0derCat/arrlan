use std::iter::zip;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Debug, Clone)]
pub struct Matrix {
    list: Vec<i64>,
    dimensions: Vec<usize>,
}
impl Matrix {
    pub fn new(list: Vec<i64>) -> Self {
        Self {
            list: list,
            dimensions: Vec::new(),
        }
    }
    pub fn len(&self) -> usize {
        self.list.len()
    }
    pub fn sum(&self) -> i64 {
        self.list.iter().sum()
    }
    pub fn product(&self) -> i64 {
        self.list.iter().product()
    }
    fn op(self, other: Self, f: fn(i64, i64) -> i64) -> Matrix {
        Matrix {
            list: zip(self.list, other.list)
                .map(|(a, b)| f(a, b))
                .rev()
                .collect(),
            dimensions: self.dimensions,
        }
    }
}
impl Add for Matrix {
    type Output = Matrix;
    fn add(self, other: Matrix) -> Matrix {
        self.op(other, |a, b| a + b)
    }
}
impl Sub for Matrix {
    type Output = Matrix;
    fn sub(self, other: Matrix) -> Matrix {
        self.op(other, |a, b| a - b)
    }
}
impl Mul for Matrix {
    type Output = Matrix;
    fn mul(self, other: Matrix) -> Matrix {
        self.op(other, |a, b| a * b)
    }
}
impl Div for Matrix {
    type Output = Matrix;
    fn div(self, other: Matrix) -> Matrix {
        self.op(other, |a, b| a / b)
    }
}
