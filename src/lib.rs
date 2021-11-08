#![allow(incomplete_features, unused_macros)]
#![feature(generic_const_exprs, adt_const_params, type_name_of_val)]

mod bool_trait {
    // From https://internals.rust-lang.org/t/const-generics-where-restrictions/12742/6
    pub trait True {}
    pub struct If<const B: bool>;
    impl True for If<true> {}
    pub const fn or(b: bool, c: bool) -> bool {
        b || c
    }
}

mod instructions;

use bool_trait::*;
pub use instructions::*;

pub struct Row<const REMAINING: usize, const COMPLETE: usize>;

impl<const REM: usize, const COM: usize> Row<REM, COM> {
    #[must_use]
    pub fn apply<I: Instruction<INPUT, OUTPUT>, const INPUT: isize, const OUTPUT: usize>(
        &mut self,
        _i: I,
    ) -> Row<{ (REM as isize - INPUT) as usize }, { COM + OUTPUT }>
    where
        I: Allowed<REM, COM>,
        If<{ REM as isize >= INPUT }>: True,
    {
        Row
    }
}

impl<const LEN: usize> Row<0, LEN> {
    pub fn complete(self) -> Row<LEN, 0> {
        Row
    }
}

#[macro_export]
macro_rules! knit {
    (>> $($ins: expr),+ $(=> $($ins2: expr),+)*) => {
        knit!{ Row::<0, 0>, $($ins),+ $(=> $($ins2),+)*}
    };
    ($row: expr, $($($ins: expr),+)=>+) => {
        $row $( $(.apply($ins))+.complete() )+
    }
}

#[allow(unused)]
mod tests {
    use super::*;

    /// ```
    /// use knitcheck::*;
    ///
    /// Row::<0, 0>.complete();
    /// ```
    fn empty_row_is_complete() {}

    /// ```compile_fail
    /// use knitcheck::*;
    ///
    /// Row::<1, 0>.complete();
    /// ```
    fn nonempty_row_is_incomplete() {}

    /// ```
    /// use knitcheck::*;
    ///
    /// Row::<0, 1>.complete();
    /// ```
    fn finished_row_is_complete() {
    }

    #[test]
    fn can_cast_on_at_start_of_row() {
        Row::<5, 0>.apply(CastOn::<17>);
    }

    #[test]
    fn can_cast_on_at_end_of_row() {
        Row::<0, 5>.apply(CastOn::<52>);
    }

    /// ```compile_fail
    /// use knitcheck::*;
    ///
    /// Row::<0, 0>.apply(CastOn::<5>).apply(Knit::<2>).complete();
    /// ```
    fn part_knitted_row_is_incomplete() {}

    /// ```compile_fail
    /// use knitcheck::*;
    ///
    /// Row::<0, 0>.apply(CastOn::<5>).complete();
    /// ```
    fn complete_unknitted_row_is_incomplete() {}

    /// ```compile_fail
    /// use knitcheck::*;
    ///
    /// Row::<0, 5>.apply(Knit::<5>);
    /// ```
    fn cannot_knit_with_no_remaining_stitches() {}

    /// ```compile_fail
    /// use knitcheck::*;
    ///
    /// Row::<2, 3>.apply(CastOn::<5>);
    /// ```
    fn cannot_cast_on_mid_row() {}

    /// ```compile_fail
    /// use knitcheck::*;
    ///
    /// knit! { >> CastOn::<7> };
    /// ```
    fn knit_macro_checks_single_row_is_complete() {}

    #[test]
    #[deny(unused_must_use)]
    fn knit_macro_succeeds_if_single_row_is_complete() {
        knit! { >> CastOn::<7>, Knit::<7> };
    }

    #[test]
    #[deny(unused_must_use)]
    fn knit_macro_succeeds_if_multiple_rows_are_complete() {
        knit! { 
            >> CastOn::<7>, Knit::<7>
            => Purl::<7>
        };
    }
}
