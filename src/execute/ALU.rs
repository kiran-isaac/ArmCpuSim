use crate::binary::{add_with_carry, bit_as_bool};
use crate::model::ASPR;

// The ALU is responsible for all the many arithmetic and logical instrs
// Not including multiplication or shifts, separate units
enum ALUOperation {
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

pub fn ALU(op: ALUOperation, n: u32, m: u32, c: bool) -> (u32, ASPR) {
    match op {
        ADD => {
            let (result, carry, overflow) = add_with_carry(n as u32, m, carry);
            (result, ASPR {
                n: bit_as_bool(result, 31),
                z: result == 0,
                c: carry == 1,
                v: overflow == 1,
            })
        }
    }
}