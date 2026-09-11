use std::iter::zip;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Debug)]
pub struct Matrix {
    list: Vec<i64>,
}
impl Matrix {
    pub fn new(list: Vec<i64>) -> Self {
        Self { list: list }
    }
    pub fn len(self) -> usize {
        self.list.len()
    }
}

impl Add for Matrix {
    type Output = Matrix;
    fn add(self, other: Matrix) -> Matrix {
        Matrix {
            list: zip(self.list, other.list)
                .map(|(a, b)| a + b)
                .rev()
                .collect(),
        }
    }
}
impl Sub for Matrix {
    type Output = Matrix;
    fn sub(self, other: Matrix) -> Matrix {
        Matrix {
            list: zip(self.list, other.list)
                .map(|(a, b)| a - b)
                .rev()
                .collect(),
        }
    }
}
impl Mul for Matrix {
    type Output = Matrix;
    fn mul(self, other: Matrix) -> Matrix {
        Matrix {
            list: zip(self.list, other.list)
                .map(|(a, b)| a * b)
                .rev()
                .collect(),
        }
    }
}
impl Div for Matrix {
    type Output = Matrix;
    fn div(self, other: Matrix) -> Matrix {
        Matrix {
            list: zip(self.list, other.list)
                .map(|(a, b)| a / b)
                .rev()
                .collect(),
        }
    }
}
