use crate::field::*;
use std::ops::Add;

pub trait Curve<CurveField: Field>: Add + Sized {
    fn scalar_mul(&self, scalar: CurveField) -> Self;
}

#[derive(Debug, PartialEq)]
pub struct Projective<CurveField: Field> {
    x: CurveField,
    y: CurveField,
    z: CurveField,
}

#[derive(Debug, PartialEq)]
pub struct Affine<CurveField: Field> {
    x: CurveField,
    y: CurveField,
    is_infinity: bool,
}

#[derive(Debug)]
pub struct CurvePoint<CurveField: Field> {
    point: Projective<CurveField>,
}

impl<const BASE: u32> Curve<PrimeField<BASE>> for CurvePoint<PrimeField<BASE>> {
   
    fn scalar_mul(&self, scalar: PrimeField<BASE>) -> Self {
        todo!()
    }
}

impl<const BASE: u32> Add<CurvePoint<PrimeField<BASE>>> for CurvePoint<PrimeField<BASE>> {
    type Output = CurvePoint<PrimeField<BASE>>;
    fn add(self, to_add: Self) -> Self::Output {
        let infinity = Projective {
            x: PrimeField::from(0),
            y: PrimeField::from(1),
            z: PrimeField::from(0),
        };
        if self.point == infinity {
            to_add
        } else if to_add.point == infinity {
            self
        } else {
            let (x_1, y_1, z_1) = (&self.point.x, &self.point.y, &self.point.z);
            let (x_2, y_2, z_2) = (&to_add.point.x, &to_add.point.y, &to_add.point.z);
            let (u_1, u_2) = (y_2 * z_1, y_1 * z_2);
            let (v_1, v_2) = (x_2 * z_1, x_1 * z_2);
            if v_1 == v_2 {
                if u_1 != u_2 {
                    Self { point: infinity }
                } else {
                    if y_1.is_null() {
                        Self { point: infinity }
                    } else {
                        unimplemented!()
                    }
                }
            } else {
                let u = u_1 - u_2;
                let v = v_1 - v_2;
                let w = z_1 * z_2;
                let a = u.pow(2) * w - v.pow(3) - PrimeField::from(2) * v.pow(2) * v_2;
                let x = v * a;
                let y = u * (v.pow(2) * v_2 - a) - v.pow(3) * u_2;
                let z = v.pow(3) * w;
                Self {
                    point: Projective { x, y, z },
                }
            }
        }
    }
}

impl<CurveField: Field> From<Projective<CurveField>> for CurvePoint<CurveField> {
    fn from(point: Projective<CurveField>) -> Self {
        CurvePoint { point }
    }
}

impl<CurveField: Field> From<Affine<CurveField>> for CurvePoint<CurveField> {
    fn from(from_point: Affine<CurveField>) -> Self {
        if from_point.is_infinity {
            return CurvePoint::<CurveField> {
                point: Projective::<CurveField> {
                    x: CurveField::zero(),
                    y: CurveField::one(),
                    z: CurveField::zero(),
                },
            };
        };
        CurvePoint::<CurveField> {
            point: Projective::<CurveField> {
                x: from_point.x,
                y: from_point.y,
                z: CurveField::one(),
            },
        }
    }
}

impl<CurveField: Field> From<CurvePoint<CurveField>> for Projective<CurveField> {
    fn from(CurvePoint { point }: CurvePoint<CurveField>) -> Self {
        point
    }
}

impl<CurveField: Field + PartialEq + Copy> From<CurvePoint<CurveField>> for Affine<CurveField> {
    fn from(CurvePoint { point }: CurvePoint<CurveField>) -> Self {
        match point.z.is_null() {
            true => Affine {
                x: point.x,
                y: point.y,
                is_infinity: true,
            },
            false => Affine {
                x: point.x.div(&point.z).unwrap(),
                y: point.y.div(&point.z).unwrap(),
                is_infinity: false,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition_works() {
        const N: u32 = 23;
        let p_point = Affine::<PrimeField<N>> {
            x: 2.into(),
            y: 5.into(),
            is_infinity: false,
        };
        let p = CurvePoint::<PrimeField<N>>::from(p_point);
        let q_point = Affine::<PrimeField<N>> {
            x: 12.into(),
            y: 1.into(),
            is_infinity: false,
        };
        let q = CurvePoint::<PrimeField<N>>::from(q_point);
        let r: Affine<PrimeField<N>> = (p + q).into();
        // println!("p: {:#?}", p);
        // println!("q: {:#?}", q);
        println!("r: {:#?}", r);
    }
}
