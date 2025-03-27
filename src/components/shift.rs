use crate::components::ALU::{ASPRUpdate, CalcResult};

#[allow(dead_code)]
pub enum ShiftType {
    LSL,
    LSR,
    ASR,
    ROR,
    RRX,
}

pub fn ror(value: u32, shift: u8) -> u32 {
    let shift = shift % 32; // Ensure the shift is within the range of 0-31
    (value >> shift) | (value << (32 - shift))
}

pub fn shift_with_carry(t: ShiftType, a: u32, b: u8, c: u8) -> CalcResult {
    let (result, c) = if b == 0 {
        (a, c != 0)
    } else {
        match t {
            ShiftType::LSL => {
                let (result, carry) = a.overflowing_shl(b as u32);
                (result, carry)
            }
            ShiftType::LSR => {
                let (result, carry) = a.overflowing_shr(b as u32);
                (result, carry)
            }
            ShiftType::ASR => {
                let (result, carry) = (a as i64).overflowing_shr(b as u32);
                (result as u32, carry)
            }
            ShiftType::ROR => {
                let result = ror(a, b);
                let carry = (result & (1 << 31)) != 0;
                (result, carry)
            }
            ShiftType::RRX => {
                let result = (a >> 1) | ((c as u32) << 31);
                let carry = (a & 1) != 0;
                (result, carry)
            }
        }
    };
    CalcResult {
        delay: 1,
        result,
        aspr_update: ASPRUpdate {
            n: None,
            z: None,
            c: Some(c),
            v: None,
        },
    }
}
