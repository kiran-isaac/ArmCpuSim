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
            imm1: 0,
            imm2: 0,
            setflags: true
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
            imm1: 0,
            imm2: 0,
            setflags: true
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
            imm1: 7,
            imm2: 0,
            setflags: true
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
            imm1: 2,
            imm2: 0,
            setflags: true
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
            imm1: 0x64,
            imm2: 0,
            setflags: true
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
            imm1: 0x64,
            imm2: 0,
            setflags: true
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
            imm1: 1,
            imm2: 0,
            setflags: true
        }
    )
}
