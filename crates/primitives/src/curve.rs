use crate::field::*;

pub trait Curve<CurveField: Field> {
    fn add(&self, to_add: Self) -> Self;
    fn mul(&self, to_mul: Self) -> Self;
    fn scalar_mul(&self, scalar: CurveField) -> Self;
}

pub struct Affine<CurveField: Field> {
    x: CurveField,
    y: CurveField,
    z: CurveField,
}

pub struct Projective<CurveField: Field> {
    x: CurveField,
    y: CurveField,
    is_infity: bool,
}

pub struct EllipticCurve<CurveField: Field> {
    point: Affine<CurveField>,
}

impl<const BASE: u32> Curve<PrimeField<BASE>> for EllipticCurve<PrimeField<BASE>> {
    fn add(&self, to_add: Self) -> Self {
        todo!()
    }

    fn mul(&self, to_mul: Self) -> Self {
        todo!()
    }

    fn scalar_mul(&self, scalar: PrimeField<BASE>) -> Self {
        todo!()
    }
}

impl<CurveField: Field> From<Affine<CurveField>> for EllipticCurve<CurveField> {
    fn from(_: Affine<CurveField>) -> Self {
        todo!()
    }
}

impl<CurveField: Field> From<Projective<CurveField>> for EllipticCurve<CurveField> {
    fn from(_: Projective<CurveField>) -> Self {
        todo!()
    }
}

impl<CurveField: Field> From<EllipticCurve<CurveField>> for Affine<CurveField> {
    fn from(_: EllipticCurve<CurveField>) -> Self {
        todo!()
    }
}

impl<CurveField: Field> From<EllipticCurve<CurveField>> for Projective<CurveField> {
    fn from(_: EllipticCurve<CurveField>) -> Self {
        todo!()
    }
}
