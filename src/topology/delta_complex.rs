use crate::algebra::{matrix::Mat, ring_traits::Ring};


pub struct DeltaComplex {
    boundary: Vec<Vec<usize>>,
}

impl DeltaComplex {
    pub fn new() -> DeltaComplex {
        DeltaComplex {
            boundary: Vec::new(),
        }
    }

    pub fn add(&mut self, bd: Vec<usize>) -> usize {
        self.boundary.push(bd);
        self.boundary.len() - 1
    }

    pub fn get_simplices_of_dim(&self, n: usize) -> Vec<usize> {
        let bd_len = if n==0 {
            0
        } else {
            n + 1
        };

        self.boundary.iter()
            .enumerate()
            .filter(|(_simp, bd)| bd.len() == bd_len)
            .map(|(simp, _bd)| simp)
            .collect()
    }

    pub fn get_boundary_mat<R>(&self, n: usize) -> Mat<R> 
        where R: Ring
    {
        let dom_basis = self.get_simplices_of_dim(n);
        let codom_basis = self.get_simplices_of_dim(n-1);

        let mut boundary_mat = Mat::new_with_value(codom_basis.len(), dom_basis.len(), R::zero());
        
        for col in 0..dom_basis.len() {
            let boundary_of_simplex = &self.boundary[dom_basis[col]];
            for (parity, &simp_id) in boundary_of_simplex.iter().enumerate() {
                let row = codom_basis.iter().position(|&id| id==simp_id).unwrap();
                let sgn = (-1isize).pow(TryInto::try_into(parity).unwrap());
                boundary_mat[(row,col)] += R::from(sgn) * R::one();
            }
        }
        
        boundary_mat
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boundary_mat1() {
        let mut triangle = DeltaComplex::new();
        
        let a = triangle.add(Vec::new());
        let b = triangle.add(Vec::new());
        let c = triangle.add(Vec::new());

        let ab = triangle.add(vec![a, b]);
        let bc = triangle.add(vec![b, c]);
        let ac = triangle.add(vec![a, c]);

        let abc = triangle.add(vec![bc,ac,ab]);

        let mat: Mat<i32> = triangle.get_boundary_mat(2);
        assert_eq!(mat, Mat::new(3, 1, vec![1, 1, -1]));
    }

    #[test]
    fn test_boundary_mat2() {
        let mut torus = DeltaComplex::new();
        
        let v = torus.add(Vec::new());

        let a = torus.add(vec![v, v]);
        let b = torus.add(vec![v, v]);
        let c = torus.add(vec![v, v]);

        let U = torus.add(vec![a,c,b]);
        let L = torus.add(vec![a,c,b]);

        let bd2: Mat<i32> = torus.get_boundary_mat(2);
        assert_eq!(bd2, Mat::new(3, 2, 
                vec![
                     1,  1,
                     1,  1,
                    -1, -1
                ]
            )
        );

        let bd1: Mat<i32> = torus.get_boundary_mat(1);
        assert_eq!(bd1, Mat::new(1, 3, vec![0, 0, 0])
    );
    }

    #[test]
    fn test_construct_circle() {
        let mut circle = DeltaComplex::new();
        let pt = circle.add(Vec::new());
        let edge = circle.add(vec![pt, pt]);

        println!("{:?}", circle.get_simplices_of_dim(0));
        println!("{:?}", circle.get_simplices_of_dim(1));
    }
}