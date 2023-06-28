use std::ops::{Add, Neg, Sub, AddAssign, SubAssign, Mul, MulAssign, Div, DivAssign};


pub trait Zero {
    fn zero() -> Self;
}

pub trait One {
    fn one() -> Self;
}

pub trait AddGroup: Sized + Clone
    + Add<Self, Output = Self> + AddAssign<Self> + Zero
    + Neg<Output = Self> + Sub<Self, Output = Self> + SubAssign<Self>
{}

pub trait Ring: Sized + Clone
    + AddGroup
    + Mul<Self, Output = Self> + MulAssign<Self> + One
{
    fn from(n: isize) -> Self {
        let mut val = Self::zero();
        if n > 0 {
            for _ in 0..n {
                val = val + Self::one();
            }
        } else {
            for _ in 0..(-n) {
                val = val - Self::one();
            }
        }
        val
    }
}

pub trait EuclDomain: Ring {
    fn eucl_norm(&self) -> u32;
    
    // for (q, r) = a.eucl_div(b), satisfies
    // `a = qb + r`
    fn eucl_div(&self, b: &Self) -> (Self, Self);
}

impl Zero for i32 { fn zero() -> Self { 0 } }
impl One for i32 { fn one() -> Self { 1 } }
impl AddGroup for i32 {}
impl Ring for i32 {}

impl EuclDomain for i32 {
    fn eucl_norm(&self) -> u32 {
        self.abs().try_into().unwrap()
    }

    fn eucl_div(&self, b: &Self) -> (Self, Self) {
        (self.div_euclid(*b), self.rem_euclid(*b))
    }
}

pub trait Field: EuclDomain + Div<Self, Output = Self> + DivAssign<Self> {}

impl Zero for f32 {
    fn zero() -> Self {
        0f32
    }
}
impl One for f32 {
    fn one() -> Self {
        1f32
    }
}
impl AddGroup for f32 {}
impl Ring for f32 {}
impl EuclDomain for f32 {
    fn eucl_norm(&self) -> u32 {
        0
    }

    fn eucl_div(&self, b: &Self) -> (Self, Self) {
        ((self/b), 0f32)
    }
}
impl Field for f32 {}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_i32_eucl_domain() {
        assert_eq!(8.eucl_div(&3), (2,2));
        assert_eq!((-8).eucl_div(&3), (-3,1));
    }
}