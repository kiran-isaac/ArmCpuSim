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
}
