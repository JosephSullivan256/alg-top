use std::{fmt::Display, ops::{Index, IndexMut}};

use crate::algebra::ring_traits::Ring;


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
}
