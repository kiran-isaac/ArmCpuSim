use crate::decode::{decode, I, IT::*};

#[test]
fn adc_test() {
    assert_eq!(
        decode(0x414f),
        I {
            it: ADC,
            rd: 7,
            rn: 7,
            rm: 1,
            rt: 0,
            rl: 0,
            immu: 0,
            imms: 0,
            setsflags: true
        }
    );
    assert_eq!(
        decode(0x416b),
        I {
            it: ADC,
            rd: 3,
            rn: 3,
            rm: 5,
            rt: 0,
            rl: 0,
            immu: 0,
            imms: 0,
            setsflags: true
        }
    )
}

#[test]
fn addimm_test() {
    assert_eq!(
        decode(0x1de9),
        I {
            it: ADDImm,
            rd: 1,
            rn: 5,
            rm: 0,
            rt: 0,
            rl: 0,
            immu: 7,
            imms: 0,
            setsflags: true
        }
    );
    assert_eq!(
        decode(0x1c97),
        I {
            it: ADDImm,
            rd: 7,
            rn: 2,
            rm: 0,
            rt: 0,
            rl: 0,
            immu: 2,
            imms: 0,
            setsflags: true
        }
    );

    assert_eq!(
        decode(0x3064),
        I {
            it: ADDImm,
            rd: 0,
            rn: 0,
            rm: 0,
            rt: 0,
            rl: 0,
            immu: 0x64,
            imms: 0,
            setsflags: true
        }
    );

    assert_eq!(
        decode(0x3064),
        I {
            it: ADDImm,
            rd: 0,
            rn: 0,
            rm: 0,
            rt: 0,
            rl: 0,
            immu: 0x64,
            imms: 0,
            setsflags: true
        }
    );
    assert_eq!(
        decode(0x3401),
        I {
            it: ADDImm,
            rd: 4,
            rn: 4,
            rm: 0,
            rt: 0,
            rl: 0,
            immu: 1,
            imms: 0,
            setsflags: true
        }
    )
}

// Only covers a couple of these but should be fine as the decode is the same
// AND,
// EOR,
// LSLReg,
// LSRReg,
// ASRReg,
// ADC,
// SBC,
// ROR,
// TST,
// RSB,
// CMPReg, // T1
// CMN,
// ORR,
// MUL,
// BIC,
// MVN,
#[test]
fn dp_test() {
    assert_eq!(
        decode(0x4001),
        I {
            it: AND,
            rd: 1,
            rn: 1,
            rm: 0,
            rt: 0,
            rl: 0,
            immu: 0,
            imms: 0,
            setsflags: true
        }
    );
    assert_eq!(
        decode(0x42d2),
        I {
            it: CMN,
            rd: 2,
            rn: 2,
            rm: 2,
            rt: 0,
            rl: 0,
            immu: 0,
            imms: 0,
            setsflags: true
        }
    );
    assert_eq!(
        decode(0x43f0),
        I {
            it: MVN,
            rd: 0,
            rn: 0,
            rm: 6,
            rt: 0,
            rl: 0,
            immu: 0,
            imms: 0,
            setsflags: true
        }
    );
    assert_eq!(
        decode(0x434d),
        I {
            it: MUL,
            rd: 5,
            rn: 5,
            rm: 1,
            rt: 0,
            rl: 0,
            immu: 0,
            imms: 0,
            setsflags: true
        }
    );
}

#[test]
fn bl_test() {
    assert_eq!(
        decode(0xf01dff2c),
        I {
            it: BL,
            rd: 0,
            rn: 0,
            rm: 0,
            rt: 0,
            rl: 0,
            immu: 0,
            imms: 122456,
            setsflags: false
        }
    );

    assert_eq!(
        decode(0xf7e7fcb0),
        I {
            it: BL,
            rd: 0,
            rn: 0,
            rm: 0,
            rt: 0,
            rl: 0,
            imms: -100000,
            immu: 0,
            setsflags: false
        }
    );
}
