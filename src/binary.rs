pub fn matches_mask<T>(i: T, mask: T) -> bool
where
    T: Copy + std::ops::BitAnd<Output = T> + PartialEq,
{
    (i & mask) == mask
}

#[inline(always)]
fn n_ones(n: u32) -> u32 {
    (1 as u32).wrapping_shl(n) - 1
}

/// Bit range inclusive zero extend
#[inline(always)]
pub fn briz(i: u32, low: u32, high: u32) -> u32 {
    #[cfg(debug_assertions)]
    if low > high {
        panic!("briz error: {i}, {low}, {high}")
    }

    (i >> low) & n_ones(high - low + 1)
}

#[inline(always)]
pub fn bit_as_bool(i: u32, bit_index: u32) -> bool {
    (i >> bit_index) & 1 == 1
}

pub fn is_32_bit(i: u32) -> bool {
    (i >> 16) != 0
}

#[inline(always)]
pub fn hamming_weight(i: u32) -> u32 {
    i.count_ones()
}

// pub fn sign_extend(i: u32) -> u32 {}

pub fn add_with_carry(a: u32, b: u32, c: u8) -> (u32, u8, u8) {
    let unsigned_sum = (a as u64).wrapping_add(b as u64).wrapping_add(c as u64);
    let signed_sum = (a as i64).wrapping_add(b as i64).wrapping_add(c as i64);
    let result = unsigned_sum as u32;
    let carry_out = if result == (unsigned_sum as u32) {
        0
    } else {
        1
    };
    let overflowed = if i32::from_ne_bytes(result.to_ne_bytes()) == (signed_sum as i32) {
        0
    } else {
        1
    };

    (result, carry_out, overflowed)
}

#[allow(dead_code)]
pub enum ShiftType {
    LSL,
    LSR,
    ASR,
    ROR,
    RRX,
}

pub fn shift_with_carry(t: ShiftType, a: u32, b: u8, c: u8) -> (u32, bool) {
    if b == 0 {
        return (a, c != 0);
    } else {
        match t {
            ShiftType::LSL => {
                let (result, carry) = a.overflowing_shl(b as u32);
                return (result, carry);
            }
            ShiftType::LSR => {
                let (result, carry) = a.overflowing_shr(b as u32);
                return (result, carry);
            }
            ShiftType::ASR => {
                let (result, carry) = (a as i64).overflowing_shr(b as u32);
                return (result as u32, carry);
            }
            ShiftType::ROR => {
                let result = ror(a, b);
                let carry = (result & (1 << 31)) != 0;
                return (result, carry);
            }
            ShiftType::RRX => {
                let result = (a >> 1) | ((c as u32) << 31);
                let carry = (a & 1) != 0;
                return (result, carry);
            }
        }
    }
}

fn ror(value: u32, shift: u8) -> u32 {
    let shift = shift % 32; // Ensure the shift is within the range of 0-31
    (value >> shift) | (value << (32 - shift))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n_ones() {
        assert_eq!(0, n_ones(0u32));
        assert_eq!(1, n_ones(1u32));
        assert_eq!(3, n_ones(2u32));
        assert_eq!(2147483647, n_ones(31u32));
    }

    #[test]
    fn test_get_bit_range() {
        assert_eq!(briz(0xb084, 12, 15), 0xb);
        assert_eq!(briz(0xb084, 8, 11), 0);
        assert_eq!(briz(0xb084, 4, 7), 8);
        assert_eq!(briz(0xb084, 0, 3), 4);

        assert_eq!(7, briz(0xf345fb3c, 29, 31));
    }

    #[test]
    fn test_add3() {
        assert_eq!((0x0, true), add_with_carry(0xfffffff0, 0xf, 1));
        assert_eq!((0xffffffff, false), add_with_carry(0xfffffff0, 0xf, 0));
    }
}
