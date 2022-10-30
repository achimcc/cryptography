use crate::field::*;

pub trait Curve<CurveField: Field> {
    fn add(&self, to_add: &Self) -> Self;
    fn scalar_mul(&self, scalar: CurveField) -> Self;
}

pub struct Projective<CurveField: Field> {
    x: CurveField,
    y: CurveField,
    z: CurveField,
}

pub struct Affine<CurveField: Field> {
    x: CurveField,
    y: CurveField,
    is_infity: bool,
}

pub struct CurvePoint<CurveField: Field> {
    point: Projective<CurveField>,
}

impl<const BASE: u32> Curve<PrimeField<BASE>> for CurvePoint<PrimeField<BASE>> {
    fn add(&self, to_add: &Self) -> Self {
        let (x_p, y_p, z_p) = (&self.point.x, &self.point.y, &self.point.z);
        let (x_q, y_q, z_q) = (&to_add.point.x, &to_add.point.y, &to_add.point.z);
        let x_r = ((x_p * z_q) - (x_q * z_p)) * ((z_p * z_q) * ((y_p * z_q) - (y_q * z_p))).pow(2)
            - (x_p * z_q - x_q * z_p).pow(2) * (x_p * z_q + x_q * z_p);
        unimplemented!()
    }

    fn scalar_mul(&self, scalar: PrimeField<BASE>) -> Self {
        todo!()
    }
}

impl<CurveField: Field> From<Projective<CurveField>> for CurvePoint<CurveField> {
    fn from(point: Projective<CurveField>) -> Self {
        CurvePoint { point }
    }
}

impl<CurveField: Field> From<Affine<CurveField>> for CurvePoint<CurveField> {
    fn from(from_point: Affine<CurveField>) -> Self {
        if from_point.is_infity {
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
                is_infity: true,
            },
            false => Affine {
                x: point.x.div(&point.z).unwrap(),
                y: point.y.div(&point.z).unwrap(),
                is_infity: false,
            },
        }
    }
}
