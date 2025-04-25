use super::*;

impl OoOSpeculative {
    pub(super) fn decode(&mut self) {
        if let Some((pc, fb_entry)) = &self.fb {
            let pc = *pc;
            let i = decode(*fb_entry);
            let i_as_mops = decode2(i);
            let mops_len = i_as_mops.len();

            for (i, mop) in i_as_mops.into_iter().enumerate() {
                // Only increment PC if this is the last MOP in the stream
                if i < mops_len - 1 {
                    self.iq.push_back(InstructionQueueEntry {
                        i: mop,
                        pc,
                    });
                } else {
                    self.iq.push_back(InstructionQueueEntry {
                        i: mop,
                        pc,
                    });
                }
            }

            // Consume from buffer
            self.fb = None;
        }
    }
}
