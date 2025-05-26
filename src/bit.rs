use std::ops::{BitAnd, BitOr, BitXor, Not};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Bit {
    ZERO,
    ONE,
}

impl BitAnd for Bit {
    type Output = Bit;

    fn bitand(self, rhs: Self) -> Self::Output {
        match self {
            Bit::ONE => rhs,
            Bit::ZERO => Bit::ZERO,
        }
    }
}

impl Not for Bit {
    type Output = Bit;

    fn not(self) -> Self::Output {
        match self {
            Self::ONE => Self::ZERO,
            Self::ZERO => Self::ONE,
        }
    }
}

impl BitOr for Bit {
    type Output = Bit;

    fn bitor(self, rhs: Self) -> Self::Output {
        match self {
            Bit::ZERO => rhs,
            Bit::ONE => Bit::ONE,
        }
    }
}

impl BitXor for Bit {
    type Output = Bit;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match self {
            Bit::ONE => !rhs,
            Bit::ZERO => rhs,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::Bit;

    #[test]
    fn test_bitand() {
        assert_eq!(Bit::ONE & Bit::ONE, Bit::ONE);
        assert_eq!(Bit::ONE & Bit::ZERO, Bit::ZERO);
        assert_eq!(Bit::ZERO & Bit::ONE, Bit::ZERO);
        assert_eq!(Bit::ZERO & Bit::ZERO, Bit::ZERO);
    }

    #[test]
    fn test_bitor() {
        assert_eq!(Bit::ONE | Bit::ONE, Bit::ONE);
        assert_eq!(Bit::ONE | Bit::ZERO, Bit::ONE);
        assert_eq!(Bit::ZERO | Bit::ONE, Bit::ONE);
        assert_eq!(Bit::ZERO | Bit::ZERO, Bit::ZERO);
    }

    #[test]
    fn test_bitxor() {
        assert_eq!(Bit::ONE ^ Bit::ONE, Bit::ZERO);
        assert_eq!(Bit::ONE ^ Bit::ZERO, Bit::ONE);
        assert_eq!(Bit::ZERO ^ Bit::ONE, Bit::ONE);
        assert_eq!(Bit::ZERO ^ Bit::ZERO, Bit::ZERO);
    }

    #[test]
    fn test_not() {
        assert_eq!(!Bit::ONE, Bit::ZERO);
        assert_eq!(!Bit::ZERO, Bit::ONE);
    }
}
