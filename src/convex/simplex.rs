use std::iter::{repeat, Map};

use itertools::{Itertools, MultiProduct};

use crate::algebra::matrix::Mat;


pub struct Simplex(Vec<Mat<f32>>);

/*
 * to perform "marching simplices", we need to do the following:
 *   - [ ] iterate through cubes in space
 *   - [ ] iterate through Kuhn triangulation of cube
 *   - [ ] intersect simplex with hyperplane approximation of phi
 */

pub fn cube_iterator(dim: usize, len: usize) -> impl Iterator<Item=Mat<f32>>
{
    (0..dim).map(|x| (0..len).map(|x| x as f32))
        .multi_cartesian_product()
        .map(move |coord| Mat::new(dim, 1, coord))
}


// pub fn iterate_cubes(pos: Mat<f32>, dim: Mat<f32>, subdivisions: usize) {
//     let dim = pos.cols;
//     (1..(usize::pow(usize::pow(2, dim), subdivisions))).map
// }

// struct CubeIterator {
//     pos: Vec<f32>,
//     dim: usize,
//     len: usize,
//     done: bool
// }

// impl CubeIterator {
//     pub fn new(dim: usize, len: usize) -> Self {
//         CubeIterator { pos: repeat(0f32).take(dim).collect(), dim, len, done: false }
//     }
// }

// impl Iterator for CubeIterator {
//     type Item = Mat<f32>;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.done {
//             return None;
//         }
//         let out = self.pos.clone();

//         self.pos[0] += 1f32;
//         let mut i = 0;
//         while self.pos[i] >= (self.len as f32) {
//             if i == self.dim-1 {
//                 self.done = true;
//                 break;
//             }
            
//             self.pos[i] = 0f32;
//             self.pos[i+1] += 1f32;

//             i += 1;
//         }
        
//         return Some(Mat::new(1, self.dim, out).transpose());
//     }
// }

/* 
 * given the bottom left corner of a n-dimension, return a list of the simplices
 * which form the "Kuhn Triangulation" of the cube
 */
// pub fn kuhn_triangulation(corner: Mat<f32>) {
//     let dim = corner.cols;
    
    
    
//     todo!()
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube_iter() {
        assert_eq!(cube_iterator(3, 2).count(), 8);
        assert_eq!(cube_iterator(4, 2).count(), 16);
        assert_eq!(cube_iterator(3, 3).count(), 27);
        assert_eq!(cube_iterator(4, 3).count(), 81);
    }
}
