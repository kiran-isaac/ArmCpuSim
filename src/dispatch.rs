use crate::instructions::{bit_as_bool, briz};
use crate::{Memory, ProcessorState, Registers};

pub fn dispatch(i: u32, state: &mut ProcessorState) {
    // match bit_range_inclusive(i, 14, 15) {
    //     0b00 => {
    //         match bit_range_inclusive(i, 11, 13) {
    //             0b
    //         }
    //     }
    //     0b01 => {

    //     }
    //     0b10 => {

    //     }
    //     0b11 => {

    //     }
    // }
}
