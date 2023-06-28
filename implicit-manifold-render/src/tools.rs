
use three_d::*;
use implicit_manifold_compute::{algebra::matrix::Mat, convex::simplex::{Simplex, kuhn_triangulation}};

pub fn mat_to_vec(v: &Mat<f32>) -> Vector3<f32> {
    Vector3 { x: v[(0,0)], y: v[(1,0)], z: v[(2,0)] }
}

pub fn simp_to_pos<'a>(s: &Simplex, index: u16) -> (impl Iterator<Item=Vector3<f32>>, impl Iterator<Item=u16>)
{
    let i0 = index;
    let i1 = index+1;
    let i2 = index+2;
    let i3 = index+3;
    (
        s.vertices.iter().map(|v| mat_to_vec(v)).collect::<Vec<Vector3<f32>>>().into_iter(),
         vec![
                i1,i2,i3,
                i0,i2,i3,
                i0,i1,i3,
                i0,i1,i2
            ].into_iter()
    )
}

pub fn kuhn_to_pos(corner: Mat<f32>) -> (Vec<Vector3<f32>>, Vec<u16>) {
    let (pos, ind): (Vec<_>, Vec<_>) = 
        kuhn_triangulation(corner).enumerate()
        .map(|(n,s)| simp_to_pos(&s, (n as u16)*4)).unzip();
    
    return (pos.into_iter().flatten().collect(), ind.into_iter().flatten().collect());
}