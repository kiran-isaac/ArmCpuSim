enum ROBStatus {
    Commit,
    Write,
    Execute,
}

struct ROBEntry {
    index: usize,
    pc: usize,
    status: ROBStatus,
    result: u32,
    destination: u8,
}