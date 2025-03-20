use crate::model::Memory;

struct LoadQueueEntry {
    address: usize,
    rob_entry: usize,
}

struct StoreQueueEntry {
    address: usize,
    rob_entry: usize,
}

fn load_store(mem: &Memory, base: u32, offset: u32) -> u32 {
    self.s
}