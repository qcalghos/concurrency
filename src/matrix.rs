use anyhow::{anyhow, Result};
use std::{
    fmt::{self, Display},
    ops::{Add, AddAssign, Mul},
};

pub struct Matrix<T> {
    pub data: Vec<T>,
    pub row: usize, //矩阵行数
    pub col: usize, //矩阵列数
}

impl<T: fmt::Debug + Sized> Matrix<T>
where
    T: Sized,
{
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Matrix {
            data: data.into(),
            row,
            col,
        }
    }
}
impl<T> fmt::Debug for Matrix<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Matrix(row={},col={},{})", self.row, self.col, self)
    }
}
impl<T> fmt::Display for Matrix<T>
where
    T: Display,
{
    //display[[1,2],[3,4],[5,6]] as {1 2,3 4,5 6}
    //display[[1,2,3],[4,5,6]] as {1 2 3,4 5 6}
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?; //两个{{表示打印的是{本身
        for i in 0..self.row {
            for j in 0..self.col {
                //打印第i行
                write!(f, "{}", self.data[i * self.col + j])?;
                if j != self.col - 1 {
                    write!(f, " ")?;
                }
            }
            //打印换行符
            if i != self.row - 1 {
                write!(f, ",")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}
//单线程方式
pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: fmt::Debug + Default + Add<Output = T> + AddAssign + Mul<Output = T> + Copy,
{
    if a.col != b.row {
        return Err(anyhow!("Matrix multiply error:a.col!=b.row"));
    }
    //这里考虑为啥不能用Vec::with_capability()初始化
    let mut data = vec![T::default(); a.row * b.col]; //a的行数乘以b的列数作为乘积矩阵的元素个数。
    for i in 0..a.row {
        for j in 0..b.col {
            for k in 0..a.col {
                data[i * b.col + j] += a.data[i * a.col + k] * b.data[k * b.col + j];
            }
        }
    }
    Ok(Matrix {
        data,
        row: a.row,
        col: b.col,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_multiply() -> Result<()> {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4, 5, 6], 3, 2);
        let c = multiply(&a, &b)?;
        assert_eq!(c.row, 2);
        assert_eq!(c.col, 2);
        assert_eq!(c.data, vec![22, 28, 49, 64]);
        assert_eq!(format!("{:?}", c), "Matrix(row=2,col=2,{22 28,49 64})");
        Ok(())
    }
}
