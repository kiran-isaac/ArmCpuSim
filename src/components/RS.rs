enum RSData {
    Register(u8),
    Data(u32),
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct RSId {
    section: u8,
    location: u8,
}

struct RS {
    busy: bool,
    j: RSData,
    k: RSData,
    /// The ROB entry
    dest: usize,
}

struct RSSet {
    n: usize,
    vec: Vec<RS>,
}
