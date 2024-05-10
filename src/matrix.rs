// use crate::vector;
// use anyhow::{anyhow,Result};
// use std::{fmt,ops::{Add,AddAssign,Mul},sync::mpsc,thread};

// const NUM_THREADS:usize=4;

use std::{
    fmt::{self, write, Debug},
    ops::{Add, AddAssign, Mul},
};

use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct Matrix<T: fmt::Debug> {
    pub data: Vec<T>,
    pub row: usize, //矩阵行数
    pub col: usize, //矩阵列数
}

// impl<T: fmt::Debug+Sized> Matrix<T> 
// where T:Sized{
//     pub fn new(data: Into<Vec<T>>, row: usize, col: usize) -> Self {
//         Matrix { data, row, col }
//     }
// }
impl<T> fmt::Display for Matrix<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{{")?;
        for i in 0..self.row {
            for j in 0..self.col {
                write!(f, "{:?}", self.data[i * self.col + j])?;
            }
            writeln!(f,",")?;
        }
        write!(f,"}}")?;
        Ok(())
    }
}
pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: fmt::Debug + Add<Output = T> + AddAssign + Mul<Output = T> + Copy,
{
    if a.col != b.row {
        return Err(anyhow!("Matrix multiply error:a.col!=b.row"));
    }
    let mut data = Vec::with_capacity(a.row * b.col); //a的行数乘以b的列数作为乘积矩阵的元素个数。
    for i in 0..a.row {
        for j in 0..b.col {
            for k in 0..a.col {
                data[i * b.col + j] += a.data[i * a.col + k] * b.data[k * b.col + j];
            }
        }
    }
    Ok(Matrix {
        data: data,
        row: a.row,
        col: b.col,
    })
}
