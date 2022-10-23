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
    fn from(point: Affine<CurveField>) -> Self {
        EllipticCurve { point }
    }
}

impl<CurveField: Field> From<Projective<CurveField>> for EllipticCurve<CurveField> {
    fn from(from_point: Projective<CurveField>) -> Self {
        if from_point.is_infity {
            return EllipticCurve::<CurveField> {
                point: Affine::<CurveField> {
                    x: CurveField::zero(),
                    y: CurveField::one(),
                    z: CurveField::zero(),
                },
            };
        };
        EllipticCurve::<CurveField> {
            point: Affine::<CurveField> {
                x: from_point.x,
                y: from_point.y,
                z: CurveField::one(),
            },
        }
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
