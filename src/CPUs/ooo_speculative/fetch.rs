use super::*;

impl OoOSpeculative {
    pub(super) fn fetch(&mut self) {
        if self.fb.is_none() {
            let fetched = self.state.mem.get_instruction(self.fetch_pc);
            self.fb = Some(FetchBufferEntry {
                i: fetched,
                pc: self.fetch_pc,
            });
            self.fetch_pc += if is_32_bit(fetched) { 4 } else { 2 };
        }
    }
}
