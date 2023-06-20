use std::iter::once;

use itertools::Itertools;

use crate::algebra::matrix::Mat;


pub struct Simplex(Vec<Mat<f32>>);

/*
 * to perform "marching simplices", we need to do the following:
 *   - [X] iterate through cubes in space
 *   - [X] iterate through Kuhn triangulation of cube
 *      - [ ] make sure that Kuhn triangulation is not just compiling but giving good result
 *   - [ ] intersect simplex with hyperplane approximation of phi
 */

pub fn cube_iterator(dim: usize, len: usize) -> impl Iterator<Item=Mat<f32>>
{
    (0..dim).map(|_| (0..len).map(|x| x as f32))
        .multi_cartesian_product()
        .map(move |coord| Mat::new(dim, 1, coord))
}


/* 
 * given the bottom left corner of a n-dimension, return a list of the simplices
 * which form the "Kuhn Triangulation" of the cube
 */
pub fn kuhn_triangulation(corner: Mat<f32>) -> impl Iterator<Item=Simplex>
{
    let dim = corner.rows;
    
    (0..dim).permutations(dim)
        .map(
            move |perm| {
                let mut directions = once(corner.clone())
                    .chain(
                        perm.into_iter().map(move |i| Mat::<f32>::ei(dim, i))
                    ).collect::<Vec<Mat<f32>>>();
                
                for i in 1..(dim+1) {
                    let prev = directions[i-1].clone();
                    directions[i] += prev;
                }

                Simplex(directions)
            }   
        )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube_iter() {
        for cube in cube_iterator(2, 2) {
            println!("{}", cube);
        }
        assert_eq!(cube_iterator(3, 2).count(), 8);
        assert_eq!(cube_iterator(4, 2).count(), 16);
        assert_eq!(cube_iterator(3, 3).count(), 27);
        assert_eq!(cube_iterator(4, 3).count(), 81);
    }

    #[test]
    fn test_kuhn_triangulation() {
        // for simp in kuhn_triangulation(Mat::new_with_value(3, 1, 0f32)) {
        //     println!("simp");
        //     for vert in simp.0 {
        //         println!("{}", vert);
        //     }
        // }

        let mut factorial = 1;
        for i in 1..6 {
            factorial *= i;
            assert_eq!(kuhn_triangulation(Mat::new_with_value(i, 1, 0f32)).count(), factorial);
        }
    }
}
