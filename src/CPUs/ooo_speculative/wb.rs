use super::*;

impl OoOSpeculative {
    pub(super) fn wb(&mut self) {
        // Decrease all delays, and add to ready queue if they are 0
        let mut free_slots = CDB_WIDTH;
        let mut new_to_broadcast = Vec::new();
        for (delay, record) in self.to_broadcast.iter_mut() {
            if free_slots <= 0 {break;}
            if *delay > 1 {
                *delay -= 1;
                new_to_broadcast.push((*delay, record.clone()));
            } else {
                record.valid = true;
                self.cdb.push_back(record.clone());
                free_slots -= 1;
            }
        }
        self.to_broadcast = new_to_broadcast;


        // Broadcast the first {CDB_WIDTH} cdb records to everything that needs it
        for _ in 0..CDB_WIDTH {
            if let Some(record) = self.cdb.pop_front() {
                let rob_entry = self.rob.get(record.rob_number).clone();
                self.rob.set_value_and_ready(record.rob_number, record.result);
                if record.halt {
                    self.rob.set_halt(record.rob_number);
                }
                assert_eq!(rob_entry.status, ROBStatus::Execute);

                match rob_entry.dest {
                    ROBEntryDest::Address(_) => {
                        // There should be no reservation stations waiting on this thing
                        self.rs_control.assert_none_waiting_for_rob(record.rob_number);
                        self.rs_mul.assert_none_waiting_for_rob(record.rob_number);
                        self.rs_alu_shift.assert_none_waiting_for_rob(record.rob_number);
                        self.rs_ls.assert_none_waiting_for_rob(record.rob_number);
                    },
                    // There may be RS waiting on this thing (this could be cmp so we're waiting for flags)
                    // We will deal with flags after this
                    ROBEntryDest::None => {},
                    ROBEntryDest::AwaitingAddress => {
                        let address = record.result;
                        self.rob.set_address(record.rob_number, address);
                    }
                    ROBEntryDest::Register(n, cspr) => {
                        self.rs_control.receive_cdb_broadcast(record.rob_number, n, record.result);
                        self.rs_mul.receive_cdb_broadcast(record.rob_number, n, record.result);
                        self.rs_alu_shift.receive_cdb_broadcast(record.rob_number, n, record.result);
                        self.rs_ls.receive_cdb_broadcast(record.rob_number, n, record.result);

                        if cspr {
                            if let Some(n) = record.aspr_update.n {
                                self.rs_control.receive_cdb_broadcast(record.rob_number, 16, n as u32);
                                self.rs_mul.receive_cdb_broadcast(record.rob_number, 16, n as u32);
                                self.rs_alu_shift.receive_cdb_broadcast(record.rob_number, 16, n as u32);
                                self.rs_ls.receive_cdb_broadcast(record.rob_number, 16, n as u32);
                            }
                            if let Some(n) = record.aspr_update.z {
                                self.rs_control.receive_cdb_broadcast(record.rob_number, 17, n as u32);
                                self.rs_mul.receive_cdb_broadcast(record.rob_number, 17, n as u32);
                                self.rs_alu_shift.receive_cdb_broadcast(record.rob_number, 17, n as u32);
                                self.rs_ls.receive_cdb_broadcast(record.rob_number, 17, n as u32);
                            }
                            if let Some(n) = record.aspr_update.c {
                                self.rs_control.receive_cdb_broadcast(record.rob_number, 18, n as u32);
                                self.rs_mul.receive_cdb_broadcast(record.rob_number, 18, n as u32);
                                self.rs_alu_shift.receive_cdb_broadcast(record.rob_number, 18, n as u32);
                                self.rs_ls.receive_cdb_broadcast(record.rob_number, 18, n as u32);
                            }
                            if let Some(n) = record.aspr_update.v {
                                self.rs_control.receive_cdb_broadcast(record.rob_number, 19, n as u32);
                                self.rs_mul.receive_cdb_broadcast(record.rob_number, 19, n as u32);
                                self.rs_alu_shift.receive_cdb_broadcast(record.rob_number, 19, n as u32);
                                self.rs_ls.receive_cdb_broadcast(record.rob_number, 19, n as u32);
                            }
                        }
                    }
                }
            }
        }
    }
}
