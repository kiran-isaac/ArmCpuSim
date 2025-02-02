use std::io::{Read, Write};

use crate::{ProcessorState, I};

/// HALT = 0,
/// PUTS = 1,
/// GETS = 2,
pub fn syscall(i: &I, state: &mut ProcessorState) {
    match i.immu {
        0 => {
            state.halting = state.regs.get(0) as i32;
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
        2 => {
            // flush incase of output on same line
            std::io::stdout().flush().unwrap();
            let addr = state.regs.get(0) as u32;
            let mut i = 0;
            loop {
                let c = std::io::stdin().bytes().next().unwrap().unwrap();
                if c == 10 {
                    break;
                }
                state.mem.set_byte_nolog(addr + i, c);
                i += 1;
            }
            // add null terminator
            state.mem.set_byte_nolog(addr + i, 0);
        }
        _ => unimplemented!(),
    }
}
