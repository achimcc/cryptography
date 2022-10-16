pub trait Field {
    fn add(&self, to_add: Self) -> Self;
    fn sub(&self, to_sub: Self) -> Self;
    fn mul(&self, to_mul: Self) -> Self;
    fn inv(&self) -> Option<Self>
    where
        Self: Sized;
    fn div(&self, to_div: Self) -> Option<Self>
    where
        Self: Sized;
}

#[derive(PartialEq, Eq, Debug)]
pub struct PrimeField<const BASE: u32> {
    value: u32,
}

impl<const BASE: u32> Field for PrimeField<BASE> {
    fn add(&self, to_add: Self) -> Self {
        ((self.value + to_add.value) % BASE).into()
    }

    fn sub(&self, to_sub: Self) -> Self {
        ((((self.value as i32 - to_sub.value as i32) % BASE as i32) + BASE as i32) as u32 % BASE)
            .into()
    }

    fn mul(&self, to_mul: Self) -> Self {
        ((self.value * to_mul.value) % BASE).into()
    }

    fn inv(&self) -> Option<Self>
    where
        Self: Sized,
    {
        if self.value == 0 {
            return None;
        }
        for x in 1..BASE {
            if (x * self.value) % BASE == 1 {
                return Some(x.into());
            }
        }
        None
    }

    fn div(&self, to_div: Self) -> Option<Self>
    where
        Self: Sized,
    {
        if let Some(inv) = to_div.inv() {
            return Some(self.mul(inv).into());
        }
        None
    }
}

impl<const BASE: u32> From<u32> for PrimeField<BASE> {
    fn from(value: u32) -> Self {
        Self {
            value: (value % BASE),
        }
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
        assert_eq!(a.div(b).unwrap(), PrimeField::<P>::from(4));
    }
}
