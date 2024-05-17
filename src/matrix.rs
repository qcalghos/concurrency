use crate::{dot_product, Vector};
use anyhow::{anyhow, Result};
use std::{
    fmt::{self, Display},
    ops::{Add, AddAssign, Mul},
    sync::mpsc,
    thread,
};

const NUM_THREADS: usize = 4;

pub struct Matrix<T> {
    pub data: Vec<T>,
    pub row: usize, //矩阵行数
    pub col: usize, //矩阵列数
}
//多线程接收的输入和发送的输出
pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}
pub struct MsgOutput<T> {
    idx: usize,
    value: T, //计算结果
}
pub struct Msg<T> {
    input: MsgInput<T>,
    sender: oneshot::Sender<MsgOutput<T>>,
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
impl<T> Mul for Matrix<T>
where
    T: fmt::Debug + Default + Add<Output = T> + AddAssign + Mul<Output = T> + Copy + Send + 'static,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        multiply(&self, &rhs).expect("Matrix<T> multiply error.")
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
impl<T> MsgInput<T> {
    pub fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        Self { idx, row, col }
    }
}
impl<T> Msg<T> {
    pub fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { input, sender }
    }
}
//多线程方式
pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: fmt::Debug + Default + Add<Output = T> + AddAssign + Mul<Output = T> + Copy + Send + 'static,
{
    if a.col != b.row {
        return Err(anyhow!("Matrix multiply error:a.col!=b.row"));
    }
    let senders = (0..NUM_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let value = dot_product(msg.input.row, msg.input.col)?;
                    if let Err(e) = msg.sender.send(MsgOutput {
                        idx: msg.input.idx,
                        value,
                    }) {
                        eprintln!("Send error:{}", e);
                    }
                }
                Ok::<_, anyhow::Error>(())
            });
            tx
        })
        .collect::<Vec<_>>();

    //这里考虑为啥不能用Vec::with_capability()初始化
    let matrix_len = a.row * b.col;
    let mut data = vec![T::default(); matrix_len]; //a的行数乘以b的列数作为乘积矩阵的元素个数。
    let mut receivers = Vec::with_capacity(matrix_len);
    for i in 0..a.row {
        for j in 0..b.col {
            //a的第i行第一个元素:i*a.col,第i行最后一个元素(i+1)*a.col-1
            let row = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
            //b的第k列第一个一个元素:k*b.col,第k列最后一个元素
            let col_data = b.data[j..]
                .iter()
                .step_by(b.col)
                .copied()
                .collect::<Vec<_>>();
            let col = Vector::new(col_data);
            let idx = i * b.col + j;
            let input = MsgInput::new(idx, row, col);
            let (tx, rx) = oneshot::channel();
            let msg = Msg::new(input, tx);
            if let Err(e) = senders[idx % NUM_THREADS].send(msg) {
                eprintln!("Send error:{}", e);
            }
            receivers.push(rx);
        }
    }
    for rx in receivers {
        let ret = rx.recv()?;
        data[ret.idx] = ret.value;
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
        let c = a * b;
        assert_eq!(c.row, 2);
        assert_eq!(c.col, 2);
        assert_eq!(c.data, vec![22, 28, 49, 64]);
        assert_eq!(format!("{:?}", c), "Matrix(row=2,col=2,{22 28,49 64})");
        Ok(())
    }
    #[test]
    #[should_panic(expected = "Matrix multiply error:a.col!=b.row")]
    fn test_can_not_multiply() {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4], 2, 2);
        let _ = a * b;
    }
}
