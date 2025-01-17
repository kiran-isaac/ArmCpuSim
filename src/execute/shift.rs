use crate::instructions::{bit_range_inclusive, matches_mask};
use crate::ProcessorState;

// Shift (immediate), add, subtract, move, and compare
pub fn shift(i: u32, state : &mut ProcessorState) {

}

pub fn LSL(i: u32, state : &mut ProcessorState) {
    #[cfg(debug_assertions)]
    assert!(matches_mask(i, 0b00000 << 10));

    let imm5 = bit_range_inclusive(i, 6, 10);
    let m_reg = bit_range_inclusive(i, 3, 5);
    let d_reg = bit_range_inclusive(i, 0, 2);
    state.regs.set_register(d_reg, state.regs.get_register(m_reg) << imm5)
}