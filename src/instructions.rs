use super::*;

/// A knitting instruction, which removes INPUT stitches from the remaining amount, and adds OUTPUT
/// stitches to the completed amount.
///
/// Any type which implements this should also implement [Allowed].
pub trait Instruction<const INPUT: isize, const OUTPUT: usize> {}

/// A guard to allow instructions when a row is in a certain state (e.g. at the start or end of a
/// row)
pub trait Allowed<const REMAINING: usize, const COMPLETE: usize> {}

pub struct Knit<const N: usize>;
pub struct Purl<const N: usize>;
pub struct CastOn<const N: usize>;

impl<const N: usize> Instruction<{ N as isize }, N> for Knit<N> {}
impl<const N: usize> Instruction<{ N as isize }, N> for Purl<N> {}
impl<const N: usize> Instruction<{ -(N as isize) }, 0> for CastOn<N> {}

/// A macro to allow an instruction to be applied at any point in the row
macro_rules! allow_anywhere {
    // Input like Knit<const I: usize, const J: isize>
    // where $($t),* is I,J and $(ty),* is usize, isize
    //
    // allow_anywhere!(Knit<const I: usize>) turns to
    // impl<const M: usize, const N: usize, const I: usize> Allowed<M,N> for Knit<I> {}
    ($name:ident $(<$(const $t:ident: $ty:ty),+>)?) => {
        impl<const M: usize, const N: usize $(, $(const $t:$ty)+)?> Allowed<M,N> for $name$(<$($t),+>)? {}
    }
}

allow_anywhere!(Knit<const I: usize>);
allow_anywhere!(Purl<const I: usize>);
impl<const REMAINING: usize, const COMPLETE: usize, const I: usize> Allowed<REMAINING, COMPLETE>
    for CastOn<I>
where
    If<{ or(REMAINING == 0, COMPLETE == 0) }>: True,
{
}

