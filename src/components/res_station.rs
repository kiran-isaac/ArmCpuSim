enum ResStationOp {

}

enum ResStationArg {
    Value(u32),
    ROBEntry(u32),
}

struct ResStation {
    j: ResStationArg,
    k: ResStationArg,
    dest: usize,
    busy: bool,

}