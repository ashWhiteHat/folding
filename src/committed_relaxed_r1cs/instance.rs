use crate::matrix::DenseVectors;

use zkstd::common::CurveAffine;

pub(crate) struct Instance<C: CurveAffine> {
    pub(crate) overline_e: C,
    pub(crate) u: C::Scalar,
    pub(crate) overline_w: C,
    pub(crate) x: DenseVectors<C::Scalar>,
}

impl<C: CurveAffine> Instance<C> {
    pub(crate) fn get(&self) -> (C, C::Scalar, C, DenseVectors<C::Scalar>) {
        (self.overline_e, self.u, self.overline_w, self.x.clone())
    }
}
