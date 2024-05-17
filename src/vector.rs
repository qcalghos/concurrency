use anyhow::{anyhow, Result};
use std::ops::{Add, AddAssign, Deref, Mul};
pub struct Vector<T> {
    data: Vec<T>,
}
impl<T> Deref for Vector<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Self { data: data.into() }
    }
}

//多线程情况下，传的参数有所有权
pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T>,
{
    if a.len() != b.len() {
        return Err(anyhow!("Dot product error:a.len()!=b.len()"));
    }
    let sum = a
        .iter()
        .zip(b.iter())
        .fold(T::default(), |acc, (x, y)| acc + *x * *y);
    Ok(sum)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dot_multiply() -> Result<()> {
        let a = Vector::new([1, 2, 3]);
        let b = Vector::new([1, 2, 3]);
        assert_eq!(14, dot_product(a, b)?);
        Ok(())
    }
}
