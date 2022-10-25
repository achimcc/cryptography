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

pub struct CurvePoint<CurveField: Field> {
    point: Affine<CurveField>,
}

impl<const BASE: u32> Curve<PrimeField<BASE>> for CurvePoint<PrimeField<BASE>> {
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

impl<CurveField: Field> From<Affine<CurveField>> for CurvePoint<CurveField> {
    fn from(point: Affine<CurveField>) -> Self {
        CurvePoint { point }
    }
}

impl<CurveField: Field> From<Projective<CurveField>> for CurvePoint<CurveField> {
    fn from(from_point: Projective<CurveField>) -> Self {
        if from_point.is_infity {
            return CurvePoint::<CurveField> {
                point: Affine::<CurveField> {
                    x: CurveField::zero(),
                    y: CurveField::one(),
                    z: CurveField::zero(),
                },
            };
        };
        CurvePoint::<CurveField> {
            point: Affine::<CurveField> {
                x: from_point.x,
                y: from_point.y,
                z: CurveField::one(),
            },
        }
    }
}

impl<CurveField: Field> From<CurvePoint<CurveField>> for Affine<CurveField> {
    fn from(CurvePoint { point }: CurvePoint<CurveField>) -> Self {
        point
    }
}

impl<CurveField: Field + PartialEq + Copy> From<CurvePoint<CurveField>> for Projective<CurveField> {
    fn from(CurvePoint { point }: CurvePoint<CurveField>) -> Self {
        match point.z.is_null() {
            true => {
                return Projective {
                    x: point.x,
                    y: point.y,
                    is_infity: true,
                }
            }
            false => {
                return Projective {
                    x: point.x.div(point.z).unwrap(),
                    y: point.y.div(point.z).unwrap(),
                    is_infity: false,
                };
            }
        }
    }
}
