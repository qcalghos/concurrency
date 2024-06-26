use anyhow::Result;
use concurrency::Matrix;

fn main() -> Result<()> {
    let a = Matrix {
        data: vec![1, 2, 3, 4, 5, 6],
        row: 2,
        col: 3,
    };
    let b = Matrix {
        data: vec![10, 11, 20, 21, 30, 31],
        row: 3,
        col: 2,
    };
    let multi = a * b;
    println!("a*b:{}", multi);
    Ok(())
}
