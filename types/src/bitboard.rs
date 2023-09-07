pub struct BitBoard(pub u64);

impl BitBoard {
    const EMPTY: BitBoard = BitBoard(0);
    const FULL: BitBoard = BitBoard(!0);
}

impl std::ops::Not for BitBoard {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

macro_rules! impl_bit_ops {
    ($($t:ident::$f:ident),*) => {
        $(
        impl std::ops::$t for BitBoard {
            type Output = BitBoard;
            fn $f(self, rhs: Self) -> Self::Output {
                Self(self.0.$f(rhs.0))
            }
        })*
    };
}
macro_rules! impl_bit_assign {
    ($($t:ident::$f:ident),*) => {
        $(
            impl std::ops::$t for BitBoard {
                fn $f(&mut self, rhs: Self) {
                    self.0.$f(rhs.0);
                }
            }
        )*
    };
}

impl_bit_ops!(BitOr::bitor, BitAnd::bitand, BitXor::bitxor);
impl_bit_assign!(
    BitAndAssign::bitand_assign,
    BitOrAssign::bitor_assign,
    BitXorAssign::bitxor_assign
);
