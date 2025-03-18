use super::*;

impl OoOSpeculative {
    pub(super) fn issue(&mut self) {
        if !self.rob.is_full() && self.iq.len() > 0 {
            let iqe = self.iq.get(0).unwrap();

            if self.rob.is_full() {
                self.stall(StallReason::IssueRobFull);
                return;
            }

            let dest = self.rob.issue_receive(&iqe.i, iqe.pc);
            let issue_dest = get_issue_type(iqe.i.it.clone());

            let rs_insert = match issue_dest {
                IssueType::ALU => self.rs_alu.issue_receive(
                    &iqe.i,
                    dest,
                    &self.state.regs,
                    &self.rob.register_status,
                ),
                IssueType::MUL => self.rs_mul.issue_receive(
                    &iqe.i,
                    dest,
                    &self.state.regs,
                    &self.rob.register_status,
                ),
                IssueType::LoadStore => self.rs_ls.issue_receive(
                    &iqe.i,
                    dest,
                    &self.state.regs,
                    &self.rob.register_status,
                ),
                IssueType::Control => self.rs_control.issue_receive(
                    &iqe.i,
                    dest,
                    &self.state.regs,
                    &self.rob.register_status,
                ),
                IssueType::Shift => self.rs_shift.issue_receive(
                    &iqe.i,
                    dest,
                    &self.state.regs,
                    &self.rob.register_status,
                ),
            };

            if rs_insert.is_some() {
                self.iq.pop_front();
            } else {
                self.stall(StallReason::IssueRSFull);
            }
        }
    }
}
