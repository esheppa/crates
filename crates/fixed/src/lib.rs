use std::marker::PhantomData;
use std::ops::{Add, Div, Mul};
use std::{i128, ops::Sub};

use typenum::{
    B0, B1, Bit, Diff, Integer, IsGreater, IsGreaterOrEqual, IsLess, Max, Maximum, Prod, Sub1, Sum,
    UInt, Unsigned, Z0,
};

// TODO: based on i128 for now. consider i64 later...
// TODO: consider nonzero/nonmax for niche use with options?

#[derive(Debug, Clone, Copy)]
struct Dec<S> {
    mantissa: i128,
    scale: PhantomData<S>,
}

// no partialord/etc for now as would prefer to normalise prior?
#[derive(Debug, Clone, Copy)]
struct Rat<S, SMax> {
    num: i128,
    den: i128,
    scale: PhantomData<S>,
    max_in: PhantomData<SMax>,
}

// impl<U: Unsigned, B: Bit> Dec<U, B> {
// // consider rounding style?
// fn round<V>(self) -> Dec<V>
// where
//     V: IsLess<U> + Unsigned,
// {
//     todo!()
// }

// fn scale<V>(self) -> Option<Dec<V>>
// where
//     V: IsGreater<U> + Unsigned,
// {
//     todo!()
// }

// fn checked_add<V>(self, rhs: Dec<U>) -> Option<Dec<V>>
// where
//     V: IsGreater<U> + Unsigned,
// {
//     todo!()
// }
// fn checked_sub<V>(self, rhs: Dec<U>) -> Option<Dec<V>>
// where
//     V: IsGreater<U> + Unsigned,
// {
//     todo!()
// }

// fn mul<V, W>(self, rhs: Dec<V>) -> Dec<W>
// where V: Unsigned, W: Unsigned,
// {
//     todo!()
// }
//
// }

impl<U, UMax> Rat<U, UMax>
where
    U: Integer,
    UMax: Integer + IsGreater<U, Output = B1>,
{
    fn new(num: i128, den: i128) -> Self {
        Self {
            num,
            den,
            scale: PhantomData,
            max_in: PhantomData,
        }
    }

    fn dec<V>(self) -> Dec<Sum<U, V>>
    where
        V: Integer + IsGreaterOrEqual<UMax, Output = B1>,
        U: Add<V>,
    {
        Dec {
            mantissa: self.num * 10i128.pow(V::to_i32().try_into().unwrap()) / self.den,
            scale: PhantomData,
        }
    }
}

/// rounding required prior to check if the scale is different
impl<U> PartialEq for Dec<U>
where
    U: Integer,
{
    fn eq(&self, other: &Self) -> bool {
        self.mantissa == other.mantissa
    }
}

/// rounding required prior to check if the scale is different
impl<U> Eq for Dec<U> where U: Integer {}

/// rounding required prior to check if the scale is different
impl<U> PartialOrd for Dec<U>
where
    U: Integer,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// rounding required prior to check if the scale is different
impl<U> Ord for Dec<U>
where
    U: Integer,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.mantissa.cmp(&other.mantissa)
    }
}

impl<U> Dec<U>
where
    U: Integer,
{
    fn new(mantissa: i128) -> Self {
        Self {
            mantissa,
            scale: PhantomData,
        }
    }
    // fn mul<Ur>(self, rhs: Dec<Ur>) -> Dec<Sum<U, Ur>>
    // where
    //     Ur: Integer,
    //     U: Add<Ur>,
    // {
    //     Dec {
    //         mantissa: self.mantissa * rhs.mantissa,
    //         scale: PhantomData,
    //     }
    // }
}

/// rounding required prior to op if the scale is different
impl<U> Add for Dec<U>
where
    U: Integer,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Dec {
            mantissa: self.mantissa + rhs.mantissa,
            scale: PhantomData,
        }
    }
}

/// rounding required prior to op if the scale is different
impl<U> Sub for Dec<U>
where
    U: Integer,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Dec {
            mantissa: self.mantissa - rhs.mantissa,
            scale: PhantomData,
        }
    }
}

/// multiplication explicitly changes the scale to make the operation cheap
/// normally the value will then be rounded. There are no restrictions 
/// on the rounding, but there will be data loss if the scale is too
/// restrictive
impl<U, V> Mul<Dec<V>> for Dec<U>
where
    U: Integer + Add<V>,
    V: Integer,
{
    type Output = Dec<Sum<U, V>>;

    fn mul(self, rhs: Dec<V>) -> Self::Output {
        Dec {
            mantissa: self.mantissa * rhs.mantissa,
            scale: PhantomData,
        }
    }
}

/// division returns a new "ratio" type which preserves the operand mantissas.
/// this can then be roudned back to a decimal.
/// when rounding the scale chosen will need to be at least as large
/// as the maximum scale of the operands
impl<U, V> Div<Dec<V>> for Dec<U>
where
    U: Integer + Sub<V> + Max<V>,
    V: Integer,
{
    type Output = Rat<Diff<U, V>, Maximum<U, V>>;

    fn div(self, rhs: Dec<V>) -> Self::Output {
        Rat {
            num: self.mantissa,
            den: rhs.mantissa,
            scale: PhantomData,
            max_in: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use typenum::{N1, P1, P2, Z0};

    use super::*;

    #[test]
    fn test_add() {
        let one_point_five = Dec::<P1>::new(15);
        let three = Dec::<P1>::new(30);
        let four_point_five = Dec::<P2>::new(450);
        assert_eq!(one_point_five.mul(three), four_point_five);
    }

    #[test]
    fn test_sub() {
        let one_point_five = Dec::<P1>::new(15);
        let three = Dec::<P1>::new(30);
        let four_point_five = Dec::<P2>::new(450);
        assert_eq!(one_point_five.mul(three), four_point_five);
    }

    #[test]
    fn test_mul() {
        let one_point_five = Dec::<P1>::new(15);
        let three = Dec::<P1>::new(30);
        let four_point_five = Dec::<P2>::new(450);
        assert_eq!(one_point_five.mul(three), four_point_five);
    }

    #[test]
    fn test_div() {
        let one_point_five = Dec::<P1>::new(15);
        let three = Dec::<P1>::new(30);
        let two = Rat::<Z0, P1>::new(30, 15);
        assert_eq!(three.div(one_point_five).dec::<P1>(), two.dec::<P1>());
    }
}