#[derive(Debug, PartialEq, Clone, Copy)]
pub struct EuclideanResult {
    pub d: i32,
    pub x: i32,
    pub y: i32,
}

pub fn extended_euclidean(a: i32, b: i32) -> EuclideanResult {
    let (mut old_r, mut r): (i32, i32) = (a, b);
    let (mut old_s, mut s): (i32, i32) = (1, 0);
    let (mut old_t, mut t): (i32, i32) = (0, 1);
    let mut q: i32;
    while r != 0 {
        q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
        (old_t, t) = (t, old_t - q * t);
    }
    EuclideanResult {
        d: old_r,
        x: old_t,
        y: old_s,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn euclidean_works() {
        let result = extended_euclidean(240, 46);
        println!("{:?}", result);
    }
}
