use std::{fmt::Display, ops::{Index, IndexMut, Mul, MulAssign, AddAssign, Sub, Neg, SubAssign, Add}};

use crate::algebra::ring_traits::Ring;

use super::ring_traits::{Zero, One, AddGroup};


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Mat<T> {
    rows: usize,
    cols: usize,
    entries: Vec<T>,
}

impl<T> Mat<T>
where
    T: Clone,
{
    pub fn new(rows: usize, cols: usize, entries: Vec<T>) -> Mat<T> {
        if entries.len() != rows * cols {
            panic!("ahhhh!! bad matrix")
        }
        Mat {
            rows,
            cols,
            entries,
        }
    }

    pub fn new_with_value(rows: usize, cols: usize, value: T) -> Mat<T> {
        let mut entries = Vec::new();
        for _ in 0..(rows * cols) {
            entries.push(value.clone());
        }
        Mat {
            rows,
            cols,
            entries,
        }
    }
}

impl<T> Index<(usize,usize)> for Mat<T> {
    type Output = T;

    fn index(&self, index: (usize,usize)) -> &Self::Output {
        &self.entries[index.0*self.cols + index.1]
    }
}

impl<T> IndexMut<(usize,usize)> for Mat<T> {
    fn index_mut(&mut self, index: (usize,usize)) -> &mut T {
        &mut self.entries[index.0*self.cols + index.1]
    }
}

impl<T> Mat<T>
    where T: Display
{
    fn fmt_row(&self, f: &mut std::fmt::Formatter<'_>, row: usize) -> std::fmt::Result {
        write!(f, "    [")?;
        for col in 0..(self.cols-1) {
            write!(f, "{}, ", self[(row,col)])?;
        }
        write!(f, "{}]", self[(row,self.cols-1)])?;
        Ok(())
    }
}

impl<T> Display for Mat<T>
    where T: Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[")?;
        for row in 0..(self.rows-1) {
            self.fmt_row(f, row)?;
            writeln!(f,",")?;
        }
        self.fmt_row(f, self.rows-1)?;
        writeln!(f)?;
        writeln!(f, "]")?;
        Ok(())
    }
}

impl<T> Mat<T>
where
    T: Ring,
{
    fn zero(n: usize) -> Mat<T> {
        Mat::new_with_value(n, n, T::zero())
    }

    fn one(n: usize) -> Mat<T> {
        let mut mat = Mat::new_with_value(n, n, T::zero());
        for i in 0..n {
            mat[(i, i)] = T::one()
        }
        mat
    }
}

impl<T> Add for Mat<T>
where
    T: AddGroup,
{
    type Output = Mat<T>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut clone = self.clone();
        clone += rhs;
        clone
    }
}

impl<T> AddAssign<Self> for Mat<T>
where
    T: AddGroup,
{
    fn add_assign(&mut self, rhs: Self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                self[(row, col)] += rhs[(row,col)].clone();
            }
        }
    }
}

impl<T> Neg for Mat<T>
where
    T: AddGroup,
{
    type Output = Mat<T>;

    fn neg(self) -> Self::Output {
        let mut clone = self.clone();
        for row in 0..self.rows {
            for col in 0..self.cols {
                clone[(row, col)] = -self[(row,col)].clone();
            }
        }
        clone
    }
}

impl<T> Sub for Mat<T>
where
    T: AddGroup,
{
    type Output = Mat<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl<T> SubAssign for Mat<T>
where
    T: AddGroup,
{
    fn sub_assign(&mut self, rhs: Self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                self[(row, col)] -= rhs[(row,col)].clone();
            }
        }
    }
}

impl<T> Mul for Mat<T>
where
    T: Ring,
{
    type Output = Mat<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut prod = Mat::new_with_value(self.rows, rhs.cols, T::zero());
        for row in 0..self.rows {
            for col in 0..rhs.cols {
                prod[(row, col)] = (0..self.cols)
                    .map(|i| self[(row, i)].clone() * rhs[(i, col)].clone())
                    .reduce(|acc, e| acc + e).unwrap();
            }
        }
        prod
    }
}

impl<T> MulAssign for Mat<T>
where
    T: Ring,
{
    fn mul_assign(&mut self, rhs: Self) {
        let clone = self.clone();
        self.entries = (rhs * clone).entries;
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::string::ToString;

    #[test]
    fn test_mat_display() {
        let mat = Mat::new(3, 2, vec![
            1, 2,
            3, 4,
            5, 6,
        ]);
        assert_eq!(mat.to_string(), "[\n    [1, 2],\n    [3, 4],\n    [5, 6]\n]\n");
    }

    #[test]
    fn test_mat_mul() {
        let m1 = Mat::new(2, 3, vec![
            1, 2, 3,
            4, 5, 6
        ]);
        let m2 = Mat::new(3, 1, vec![
            7,
            8,
            9
        ]);
        let m3 = Mat::new(2, 1, vec![
            50,
            122
        ]);
        assert_eq!(m1*m2, m3);
    }
}
