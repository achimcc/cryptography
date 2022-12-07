use std::ops::{Add, Mul, Sub, Rem, DivAssign};

pub trait Field {
    fn field_add(&self, to_add: &Self) -> Self;
    fn field_sub(&self, to_sub: &Self) -> Self;
    fn field_mul(&self, to_mul: &Self) -> Self;
    fn pow(&self, to_exp: u32) -> Self;
    fn is_null(&self) -> bool;
    fn one() -> Self;
    fn zero() -> Self;
    fn inv(&self) -> Option<Self>
    where
        Self: Sized;
    fn div(&self, to_div: &Self) -> Option<Self>
    where
        Self: Sized;
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct PrimeField<const BASE: u32> {
    value: u32,
}

impl<const BASE: u32> Field for PrimeField<BASE> {
    fn field_add(&self, to_add: &Self) -> Self {
        ((self.value + to_add.value) % BASE).into()
    }

    fn field_sub(&self, to_sub: &Self) -> Self {
        ((((self.value as i32 - to_sub.value as i32) % BASE as i32) + BASE as i32) as u32 % BASE)
            .into()
    }

    fn field_mul(&self, to_mul: &Self) -> Self {
        ((self.value * to_mul.value) % BASE).into()
    }

    fn is_null(&self) -> bool {
        self.value == 0
    }

    fn inv(&self) -> Option<Self>
    where
        Self: Sized,
    {
        if self.is_null() {
            return None;
        }
        for x in 1..BASE {
            if (x * self.value) % BASE == 1 {
                return Some(x.into());
            }
        }
        None
    }

    fn div(&self, to_div: &Self) -> Option<Self>
    where
        Self: Sized,
    {
        to_div.inv().map(|inv| self.mul(&inv))
    }

    fn one() -> Self {
        PrimeField { value: 1 }
    }

    fn zero() -> Self {
        PrimeField { value: 0 }
    }

    fn pow(&self, to_exp: u32) -> Self {
        let mut result = Self::one();
        for _ in 0..to_exp {
            result = &result * self;
        }
        result
    }
}

impl<'a, 'b, const BASE: u32> Add<&'b PrimeField<BASE>> for &'a PrimeField<BASE> {
    type Output = PrimeField<BASE>;

    fn add(self, other: &'b PrimeField<BASE>) -> PrimeField<BASE> {
        self.field_add(other)
    }
}

impl<const BASE: u32> Add<PrimeField<BASE>> for PrimeField<BASE> {
    type Output = PrimeField<BASE>;

    fn add(self, other: PrimeField<BASE>) -> PrimeField<BASE> {
        self.field_add(&other)
    }
}

impl<'a, 'b, const BASE: u32> Sub<&'b PrimeField<BASE>> for &'a PrimeField<BASE> {
    type Output = PrimeField<BASE>;

    fn sub(self, other: &'b PrimeField<BASE>) -> PrimeField<BASE> {
        self.field_sub(other)
    }
}

impl<const BASE: u32> Sub<PrimeField<BASE>> for PrimeField<BASE> {
    type Output = PrimeField<BASE>;

    fn sub(self, other: PrimeField<BASE>) -> PrimeField<BASE> {
        self.field_sub(&other)
    }
}

impl<const BASE: u32> Mul<PrimeField<BASE>> for PrimeField<BASE> {
    type Output = PrimeField<BASE>;

    fn mul(self, other: PrimeField<BASE>) -> PrimeField<BASE> {
        self.field_mul(&other)
    }
}

impl<const BASE: u32> Rem<u32> for PrimeField<BASE> {
    type Output = u32;

    fn rem(self, modulus: u32) -> u32 {
        self.value % modulus
    }
}

impl<const BASE: u32> DivAssign<u32> for PrimeField<BASE> {
    fn div_assign(&mut self, rhs: u32) {
        self.value /= rhs;
    }
}

impl<'a, 'b, const BASE: u32> Mul<&'b PrimeField<BASE>> for &'a PrimeField<BASE> {
    type Output = PrimeField<BASE>;

    fn mul(self, other: &'b PrimeField<BASE>) -> PrimeField<BASE> {
        self.field_mul(other)
    }
}

impl<const BASE: u32> From<u32> for PrimeField<BASE> {
    fn from(value: u32) -> Self {
        Self {
            value: (value % BASE),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Binary(Vec<bool>);

impl<const BASE: u32> From<PrimeField<BASE>> for Binary {
    fn from(value: PrimeField<BASE>) -> Binary  {
        let mut result: Vec<bool> = Vec::new();
        let mut remainder = value.clone();
        while !remainder.is_null() {
           if (remainder%2) == 0 {result.push(false)}
           else { result.push(true) } 
           remainder /= 2; 
        }
        Binary(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition_works() {
        const P: u32 = 7;
        let [a, b] = [3, 2].map(PrimeField::<P>::from);
        assert_eq!(a.add(b), PrimeField::<P>::from(5));
    }

    #[test]
    fn subtraction_works() {
        const P: u32 = 7;
        let [a, b] = [3, 6].map(PrimeField::<P>::from);
        assert_eq!(a.sub(b), PrimeField::<P>::from(4));
    }

    #[test]
    fn multiplication_works() {
        const P: u32 = 7;
        let [a, b] = [3, 6].map(PrimeField::<P>::from);
        assert_eq!(a.mul(b), PrimeField::<P>::from(4));
    }

    #[test]
    fn division_works() {
        const P: u32 = 11;
        let [a, b] = [8, 2].map(PrimeField::<P>::from);
        assert_eq!(a.div(&b).unwrap(), PrimeField::<P>::from(4));
    }

    #[test]
    fn binary_works() { 
        const P: u32 = 7;
        let a = PrimeField::<P>::from(6);
        let result:Binary = a.into();
        let binary = Binary(vec![false, true, true]);
        assert_eq!(result, binary);
    }
}
