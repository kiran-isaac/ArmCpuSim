use super::*;

impl OoOSpeculative {
    pub(super) fn decode(&mut self) {
        if self.iq.len() < 10 {
            if let Some(fb_entry) = &self.fb {
                let i = decode(fb_entry.i);
                let pc = fb_entry.pc;
                let pc_increment = if is_32_bit(fb_entry.i) { 4 } else { 2 };
                let i_as_mops = decode2(i);
                let mops_len = i_as_mops.len();

                for (i, mop) in i_as_mops.into_iter().enumerate() {
                    // Only increment PC if this is the last MOP in the stream
                    if i < mops_len - 1 {
                        self.iq.push_back(InstructionQueueEntry {
                            i: mop,
                            pc,
                            pc_increment: 0,
                        });
                    } else {
                        self.iq.push_back(InstructionQueueEntry {
                            i: mop,
                            pc,
                            pc_increment,
                        });
                    }
                }

                // Consume from buffer
                self.fb = None;
            }
        }
    }
}
