use super::*;

impl OoOSpeculative {
    pub(super) fn issue(&mut self) {
        if self.iq.len() <= 0 {
            return;
        }
        let iqe = self.iq.get(0).unwrap();

        let dest = self.rob.issue_receive(&iqe.i, iqe.pc);
        let issue_dest = get_issue_type(iqe.i.it.clone());

        let rs_insert = match issue_dest {
            IssueType::ALUSHIFT => self.rs_alu_shift.issue_receive(
                &iqe.i,
                dest,
                &self.state.regs,
                &self.rob.register_status, 
                &self.rob
            ),
            IssueType::MUL => {
                self.rs_mul
                    .issue_receive(&iqe.i, dest, &self.state.regs, &self.rob.register_status, &self.rob)
            }
            IssueType::LoadStore => {
                self.rs_ls
                    .issue_receive(&iqe.i, dest, &self.state.regs, &self.rob.register_status, &self.rob)
            }
            IssueType::Control => self.rs_control.issue_receive(
                &iqe.i,
                dest,
                &self.state.regs,
                &self.rob.register_status, 
                &self.rob
            ),
        };

        if rs_insert.is_some() {
            self.iq.pop_front();
            self.rob.issue_commit();
        } else {
            self.stall(StallReason::IssueRSFull);
        }
    }
}
