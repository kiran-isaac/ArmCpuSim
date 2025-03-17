pub enum RSData {
    Register(u8),
    Data(u32),
    None,
}

pub struct RS {
    busy: bool,
    j: RSData,
    k: RSData,
    /// The ROB entry
    dest: Option<usize>,
}

impl RS {
    pub fn new() -> Self {
        RS {
            busy: false,
            j: RSData::None,
            k: RSData::None,
            dest: None,
        }
    }
}

pub struct RSSet<const N: usize> {
    buf: [RS; N],
}

impl<const N: usize> RSSet<N> {
    pub fn new() -> RSSet<N> {
        let vec = [RS::new(); N];
        RSSet { buf: vec }
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }
}
