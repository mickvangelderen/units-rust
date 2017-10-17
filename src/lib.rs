use ::std::marker::PhantomData;
use ::std::ops;
use ::std::cmp;
use ::std::fmt;
use ::std::convert;

pub trait Unit {
    const UNIT: &'static str;
}

pub struct Measure<V, U>(V, PhantomData<U>);

#[macro_export]
macro_rules! impl_unit {
    ($UnitName:ident, $ShortName:ident, $short:expr) => (
        #[derive(Debug)]
        pub enum $UnitName {}

        impl Unit for $UnitName {
            const UNIT: &'static str = $short;
        }

        pub type $ShortName<T> = Measure<T, $UnitName>;
    )
}

impl<V, U> Measure<V, U> {
    pub fn value(self) -> V {
        self.0
    }
}

impl<V, U> convert::From<V> for Measure<V, U> {
    fn from(v: V) -> Self {
        Measure(v, PhantomData)
    }
}

impl<V: fmt::Debug, U: Unit> fmt::Debug for Measure<V, U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}{}", self.0, U::UNIT)
    }
}

// TODO: Derive other operators?
impl<V: ops::Add, U> ops::Add for Measure<V, U> {
    type Output = Measure<V::Output, U>;

    fn add(self, other: Self) -> Self::Output {
        (self.0 + other.0).into()
    }
}

// TODO: Derive other comparison operators.
impl<V: cmp::PartialEq, U> cmp::PartialEq for Measure<V, U> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

macro_rules! impl_op_raw {
    ($Op:ident, $OpMethod:ident, $L:ident, $R:ident, $O:ident) => (
        impl<V: ops::$Op> ops::$Op<$R<V>> for $L<V> {
            type Output = $O<V::Output>;

            fn $OpMethod(self, other: $R<V>) -> Self::Output {
                ops::$Op::$OpMethod(self.0, other.0).into()
            }
        }
    );
}

#[macro_export]
macro_rules! impl_op {
    ($L:ident / $R:ident = $O:ident) => (
        impl_op_raw!(Div, div, $L, $R, $O);
    );

    ($L:ident * $R:ident = $O:ident) => (
        impl_op_raw!(Mul, mul, $L, $R, $O);
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    impl_unit!(MeterUnit, Meter, "m");
    impl_unit!(SecondUnit, Second, "s");
    impl_unit!(MeterPerSecondUnit, MeterPerSecond, "m/s");
    impl_unit!(SquareMeterUnit, SquareMeter, "m^2");

    impl_op!(Meter / Second = MeterPerSecond);
    impl_op!(Meter * Meter = SquareMeter);

    #[test]
    fn it_works() {
        let m1: Meter<_> = 10f32.into();
        let m2 = Meter::from(40f32);
        assert_eq!(SquareMeter::from(400f32), m1*m2);

        assert_eq!(
            Meter::from(5000f32)/Second::from(60f32*25f32),
            MeterPerSecond::from(5000f32/60.0/25.0)
        );
    }
}
