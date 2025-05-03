use super::*;

impl<'a> OoOSpeculative<'a> {
    pub(super) fn decode(&mut self) {
        for j in 0..N_ISSUE {
            if let Some(FetchQueueEntry { pc, i, predicted_taken }) = self.fb[j] {
                let i = decode(i);
                let i_as_mops = decode2(i);

                for mop in i_as_mops {
                    self.iq.push_back(InstructionQueueEntry { i: mop, pc, predicted_taken });
                }

                // Consume from buffer
                self.fb[j] = None;
            }
        }
    }
}
