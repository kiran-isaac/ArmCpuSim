use crate::{ProcessorState, I};

/// HALT = 0,
/// PUTS = 1,
/// GETS = 2,
pub fn syscall(i: &I, state: &mut ProcessorState) {
    match i.immu {
        0 => {
            state.halt = state.regs.get(0) as i32;
        }
        1 => {
            let mut addr = state.regs.get(0) as u32;
            loop {
                let c = state.mem.get_byte_nolog(addr);
                if c == 0 {
                    break;
                }
                print!("{}", c as char);
                addr += 1;
            }
        }
        _ => unimplemented!(),
    }
}
