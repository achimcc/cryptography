use crate::field::*;

trait Curve<CurveField : Field> {
    fn add(&self, to_add: Self) -> Self;
    fn mul(&self, to_mul: Self) -> Self;
    fn scalar_mul(&self, scalar: CurveField) -> Self;
}

