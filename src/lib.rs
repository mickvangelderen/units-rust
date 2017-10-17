use ::std::marker::PhantomData;
use ::std::ops;
// use ::std::cmp;
// use ::std::fmt;
// use ::std::convert;

struct P<T>(PhantomData<T>);
struct Z;
struct N<T>(PhantomData<T>);

// TODO: Wouldn't this be cleaner?
// trait TypeAdd<RHS = Self> {
//     type Output;
// }
// impl TypeAdd for Z {
//     type Output = Z;
// }

// 0 + 0 (additive identity)
impl ops::Add for Z {
    type Output = Z;
    fn add(self, _: Z) -> Self::Output {
        unreachable!();
    }
}

// v + 0 (additive identity)
impl<T> ops::Add<Z> for P<T> {
    type Output = Self;
    fn add(self, _: Z) -> Self::Output {
        unreachable!();
    }
}

// -v + 0 (additive identity)
impl<T> ops::Add<Z> for N<T> {
    type Output = Self;
    fn add(self, _: Z) -> Self::Output {
        unreachable!();
    }
}

// 0 + v (additive identity)
impl<T> ops::Add<P<T>> for Z {
    type Output = P<T>;
    fn add(self, _: P<T>) -> Self::Output {
        unreachable!();
    }
}

// 0 + -1 (additive identity)
impl<T> ops::Add<N<T>> for Z {
    type Output = N<T>;
    fn add(self, _: N<T>) -> Self::Output {
        unreachable!();
    }
}

// v + v (addition)
impl <TL: ops::Add<TR>, TR> ops::Add<P<TR>> for P<TL> {
    type Output = P<P<<TL as ops::Add<TR>>::Output>>;
    fn add(self, _: P<TR>) -> Self::Output {
        unreachable!();
    }
}

// -v + v (addition)
impl <TL: ops::Add<TR>, TR> ops::Add<P<TR>> for N<TL> {
    type Output = <TL as ops::Add<TR>>::Output;
    fn add(self, _: P<TR>) -> Self::Output {
        unreachable!();
    }
}

// v + -v (addition)
impl <TL: ops::Add<TR>, TR> ops::Add<N<TR>> for P<TL> {
    type Output = <TL as ops::Add<TR>>::Output;
    fn add(self, _: N<TR>) -> Self::Output {
        unreachable!();
    }
}

// -v + -v (addition)
impl <TL: ops::Add<TR>, TR> ops::Add<N<TR>> for N<TL> {
    type Output = N<N<<TL as ops::Add<TR>>::Output>>;
    fn add(self, _: N<TR>) -> Self::Output {
        unreachable!();
    }
}

// Types representing the positive numbers 1, 2 and 3.

#[cfg(test)]
mod tests {
    use super::*;

    type N1 = N<Z>;
    type N2 = N<N1>;
    type N3 = N<N2>;

    type P1 = P<Z>;
    type P2 = P<P1>;
    type P3 = P<P2>;

    #[test]
    fn it_works() {
        // Run time addition. (panics)
        let _x: P3 = P1 { 0: PhantomData } + P2 { 0: PhantomData };

        // Compile time addition is the type of _y.
        let _y: <P1 as ops::Add<P2>>::Output = _x;

        // Hahahah this works!!!
        let _x: Z = N1 { 0: PhantomData } + P1 { 0: PhantomData };
    }
}
