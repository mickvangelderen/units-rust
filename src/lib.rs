use ::std::marker::PhantomData;
use ::std::ops;
use ::std::cmp;
use ::std::fmt;
use ::std::convert;

pub enum N3 {}
pub enum N2 {}
pub enum N1 {}
pub enum Z0 {}
pub enum P1 {}
pub enum P2 {}
pub enum P3 {}

pub struct Quantity<V, D1, D2>(V, PhantomData<(D1, D2)>);

pub type Length<V> = Quantity<V, P1, Z0>;

pub type Time<V> = Quantity<V, Z0, P1>;

pub type Velocity<V> = Quantity<V, P1, N1>;

macro_rules! impl_dim_mul {
    ($L:ident * $R:ident = $O:ident) => (
        impl ops::Mul<$R> for $L {
            type Output = $O;
            fn mul(self, _: $R) -> Self::Output {
                unreachable!()
            }
        }
    )
}

// impl_dim_mul!(N3 * N3 = N6);
// impl_dim_mul!(N3 * N2 = N5);
// impl_dim_mul!(N3 * N1 = N4);
impl_dim_mul!(N3 * Z0 = N3);
impl_dim_mul!(N3 * P1 = N2);
impl_dim_mul!(N3 * P2 = N1);
impl_dim_mul!(N3 * P3 = Z0);

macro_rules! impl_dim_div {
    ($L:ty, $R:ty, $O:ty) => (
        impl ops::Div<$R> for $L {
            type Output = $O;
            fn div(self, _: $R) -> Self::Output {
                unreachable!()
            }
        }
    );
}

impl_dim_div!(N3, N3, Z0);
impl_dim_div!(N3, N2, N1);
impl_dim_div!(N3, N1, N2);
impl_dim_div!(N3, Z0, N3);
// impl_dim_div!(N3, P1, N4);
// impl_dim_div!(N3, P2, N5);
// impl_dim_div!(N3, P3, N6);

impl_dim_div!(N2, N3, P1);
impl_dim_div!(N2, N2, Z0);
impl_dim_div!(N2, N1, N1);
impl_dim_div!(N2, Z0, N2);
impl_dim_div!(N2, P1, N3);
// impl_dim_div!(N2, P2, N4);
// impl_dim_div!(N2, P3, N5);

impl_dim_div!(N1, N3, P2);
impl_dim_div!(N1, N2, P1);
impl_dim_div!(N1, N1, Z0);
impl_dim_div!(N1, Z0, N1);
impl_dim_div!(N1, P1, N2);
impl_dim_div!(N1, P2, N3);
// impl_dim_div!(N1, P3, N4);

impl ops::Div<Z0> for P1 {
    type Output = N1;
    fn div(self, _: Z0) -> Self::Output {
        unreachable!()
    }
}

impl ops::Div<P1> for Z0 {
    type Output = Z0;
    fn div(self, _: P1) -> Self::Output {
        unreachable!()
    }
}

impl<V: ops::Div, Q1D1, Q1D2, Q2D1, Q2D2> ops::Div<Quantity<V, Q2D1, Q2D2>> for Quantity<V, Q1D1, Q1D2>
    where V: ops::Div,
          Q1D1: ops::Div<Q2D1>,
          Q1D2: ops::Div<Q2D2>,
{
    type Output = Quantity<
            V::Output,
            <Q1D1 as ops::Div<Q2D1>>::Output,
            <Q1D2 as ops::Div<Q2D2>>::Output
        >;

    fn div(self, other: Quantity<V, Q2D1, Q2D2>) -> Self::Output {
        (self.0/other.0).into()
    }
}

impl<V, D1, D2> convert::From<V> for Quantity<V, D1, D2> {
    fn from(v: V) -> Self {
        Quantity(v, PhantomData)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let m1: Length<_> = 10f32.into();
        let m2 = Length::from(40f32);
    }
}
