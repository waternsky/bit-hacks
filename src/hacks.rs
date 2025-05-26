//
// Only doing for u8, works for i8, u16, i16, ...
// Note: negative numbers are stored as 2's compliment
//

/// Tells whether x is even or odd
/// checks the last bit (least significant bit)
/// if 1 -> odd (true), 0 -> even (false)
/// feels weird to call even = false
pub fn parity(x: u8) -> bool {
    if x & 1 == 1 {
        println!("{x} is odd");
        return true;
    } else {
        println!("{x} is even");
        return false;
    }
}

/// Checks whether nth bit is set
/// x & (1 << n) is bitwise and
/// will give 0 only when nth bit of x is 0
pub fn is_nthbit_set(x: u8, n: u8) -> bool {
    if x & (1 << n) == 0 {
        println!("for {x:b} {n}th bit is not set");
        return false;
    } else {
        println!("for {x:b} {n}th bit is set");
        return true;
    }
}

/// Set the nth bit
/// if nth bit is 0 or changes it to 1
pub fn set_nthbit(x: u8, n: u8) -> u8 {
    println!("{x} is {x:b}, setting {n}th bit gives {}", x | (1 << n));
    x | (1 << n)
}

/// Unset the nth bit
pub fn unset_nthbit(x: u8, n: u8) -> u8 {
    println!("{x} is {x:b}, unsetting {n}th bit gives {}", x | !(1 << n));
    x & !(1 << n)
}

/// Toggle the nth bit
/// if nth bit of x is 0 make 1 and vice-versa
pub fn toggle_nthbit(x: u8, n: u8) -> u8 {
    println!("{x} is {x:b}, toggling {n}th bit gives {}", x ^ (1 << n));
    x ^ (1 << n)
}

/// 10001 -> 10000
/// 10100 -> 10000
pub fn turnoff_rightmost_1bit(x: u8) -> u8 {
    x & (x - 1)
}

/// 10001 -> 00001
/// 10010 -> 00010
/// this is actually trying to do 2's compliment (should have picked i8 for example)
pub fn isolate_rightmost_1bit(x: u8) -> u8 {
    x & (!x + 1)
}

pub fn isolate_rightmost_1bit_i8(x: i8) -> i8 {
    x & (-x)
}

/// 10100 -> 10111
/// 10001 -> 10001
pub fn rightpropagate_rightmost_1bit(x: u8) -> u8 {
    x | (x - 1)
}

/// 100001 -> 000010
/// 100111 -> 001000
pub fn isolate_rightmost_0bit(x: u8) -> u8 {
    !x & (x + 1)
}

/// 100001 -> 100011
/// 100111 -> 101111
pub fn turnon_rightmost_0bit(x: u8) -> u8 {
    x | (x + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parity() {
        assert!(parity(123));
        assert!(!parity(64));
    }

    #[test]
    fn test_is_nthbit_set() {
        assert!(is_nthbit_set(129, 0));
        assert!(!is_nthbit_set(129, 2));
        assert!(is_nthbit_set(129, 7));
        assert!(!is_nthbit_set(129, 6));
    }

    #[test]
    fn test_set_nthbit() {
        assert_eq!(set_nthbit(0b1000001, 2), 0b1000101);
        assert_eq!(set_nthbit(112, 4), 112);
    }

    #[test]
    fn test_unset_nthbit() {
        assert_eq!(unset_nthbit(0b1000001, 0), 0b1000000);
        assert_eq!(unset_nthbit(112, 4), 96);
    }

    #[test]
    fn test_toggle_nthbit() {
        assert_eq!(toggle_nthbit(0b1000001, 0), 0b1000000);
        assert_eq!(toggle_nthbit(112, 4), 96);
    }

    #[test]
    fn test_turnoff_rightmost_1bit() {
        assert_eq!(turnoff_rightmost_1bit(0b1000001), 0b1000000);
        assert_eq!(turnoff_rightmost_1bit(112), 96);
        assert_eq!(turnoff_rightmost_1bit(0b1000000), 0);
    }

    #[test]
    fn test_isolate_rightmost_1bit() {
        assert_eq!(isolate_rightmost_1bit(0b1001000), 0b0001000);
        assert_eq!(isolate_rightmost_1bit(0b1000001), 1);
    }

    #[test]
    fn test_isolate_rightmost_1bit_i8() {
        assert_eq!(isolate_rightmost_1bit_i8(0b1001000), 0b0001000);
        assert_eq!(isolate_rightmost_1bit_i8(0b1000001), 1);
        assert_eq!(isolate_rightmost_1bit_i8(-64), 0b01000000);
    }

    #[test]
    fn test_rightpropagate_rightmost_1bit() {
        assert_eq!(rightpropagate_rightmost_1bit(0b1001000), 0b1001111);
        assert_eq!(rightpropagate_rightmost_1bit(0b1000001), 0b1000001);
        assert_eq!(rightpropagate_rightmost_1bit(0b10000), 0b11111);
    }

    #[test]
    fn test_isolate_rightmost_0bit() {
        assert_eq!(isolate_rightmost_0bit(0b1111111), 0b10000000);
        assert_eq!(isolate_rightmost_0bit(0b1100111), 0b0001000);
        assert_eq!(isolate_rightmost_0bit(0b0001010), 000000001)
    }

    #[test]
    fn test_turnon_rightmost_0bit() {
        assert_eq!(turnon_rightmost_0bit(0b100100), 0b100101);
        assert_eq!(turnon_rightmost_0bit(0b100111), 0b101111);
    }
}
