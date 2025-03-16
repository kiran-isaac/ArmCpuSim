use crate::binary::{bit_as_bool, briz};

fn add_with_carry(a: u32, b: u32, c: u8) -> (u32, u8, u8) {
    let unsigned_sum = (a as u64).wrapping_add(b as u64).wrapping_add(c as u64);
    let signed_sum = (a as i64).wrapping_add(b as i64).wrapping_add(c as i64);
    let result = unsigned_sum as u32;
    let carry_out = if (result as u64) == unsigned_sum {
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

// The ALU is responsible for all the many arithmetic and logical instrs
// Not including multiplication or shifts, separate units
pub enum ALUOperation {
    ADD,
    AND,
    OR,
    EOR,
    REV,
    REV16,
    REVSH,
    UXTH,
    UXTB,
    SXTB,
    SXTH,
}

pub struct ASPRUpdate {
    pub n: Option<bool>,
    pub z: Option<bool>,
    pub c: Option<bool>,
    pub v: Option<bool>,
}

impl ASPRUpdate {
    fn no_update() -> Self {
        ASPRUpdate {
            n: None,
            z: None,
            c: None,
            v: None,
        }
    }
}

pub fn ALU(op: ALUOperation, n: u32, m: u32, c: bool) -> (u32, ASPRUpdate) {
    match op {
        ALUOperation::ADD => {
            let (result, carry, overflow) = add_with_carry(n, m, c as u8);
            (
                result,
                ASPRUpdate {
                    n: Some(bit_as_bool(result, 31)),
                    z: Some(result == 0),
                    c: Some(carry == 1),
                    v: Some(overflow == 1),
                },
            )
        }
        ALUOperation::AND => {
            let result = n & m;
            (
                result,
                ASPRUpdate {
                    n: Some(bit_as_bool(result, 31)),
                    z: Some(result == 0),
                    c: None,
                    v: None,
                },
            )
        }
        ALUOperation::OR => {
            let result = n | m;
            (
                result,
                ASPRUpdate {
                    n: Some(bit_as_bool(result, 31)),
                    z: Some(result == 0),
                    c: None,
                    v: None,
                },
            )
        }
        ALUOperation::EOR => {
            let result = n ^ m;
            (
                result,
                ASPRUpdate {
                    n: Some(bit_as_bool(result, 31)),
                    z: Some(result == 0),
                    c: None,
                    v: None,
                },
            )
        }
        ALUOperation::REV => {
            let result =
                (n & 0xff) << 24 | (n & 0xff00) << 8 | (n & 0xff0000) >> 8 | (n & 0xff000000) >> 24;
            (result, ASPRUpdate::no_update())
        }
        ALUOperation::REV16 => {
            let result =
                (n & 0xff) << 8 | (n & 0xff00) >> 8 | (n & 0xff0000) << 8 | (n & 0xff000000) >> 8;
            (result, ASPRUpdate::no_update())
        }
        ALUOperation::REVSH => {
            let result = ((n & 0xff) << 8 | (n & 0xff00) >> 8) as i16 as i32 as u32;
            (result, ASPRUpdate::no_update())
        }
        ALUOperation::UXTH | ALUOperation::UXTB => {
            let result = match op {
                ALUOperation::UXTH => n & 0xffff,
                ALUOperation::UXTB => n & 0xff,
                _ => unreachable!(),
            };
            (result, ASPRUpdate::no_update())
        }
        ALUOperation::SXTH => {
            let sign = if bit_as_bool(n, 15) { 0x80000000 } else { 0 };
            (sign + briz(n, 0, 14), ASPRUpdate::no_update())
        }
        ALUOperation::SXTB => {
            let sign = if bit_as_bool(n, 7) { 0x80000000 } else { 0 };
            (sign + briz(n, 0, 6), ASPRUpdate::no_update())
        }
    }
}
