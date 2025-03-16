use crate::binary::{bit_as_bool, hamming_weight};
/// We need to split some of the CISCy instructions into multiple instructions
use super::*;

// enum MOps {
//     // ALU instructions
//     ADD,
//     AND,
//     OR,
//     EOR,
//     REV,
//     REV16,
//     REVSH,
//     UXTH,
//     UXTB,
//     SXTB,
//     SXTH,
//     LSL,
//     LSR,
//     ASR,
//
//     // Multiply
//     MUL,
//
//     // Load store
//     LDB,
//     STB,
//     LDH,
//     STH,
//     LDW,
//     STW,
//
//     // Branch
//     B,
//     BL,
//
//     // All of the silly instructions
//     SVC,
//     DSB,
//     DMB,
//     ISB,
// }

pub fn decode2(i: I) -> Vec<I> {
    let mut vec = Vec::new();
    match i.it {
        IT::PUSH | IT::STMIA => {
            let n = hamming_weight(i.rl as u32);
            let target = match i.it {
                IT::PUSH => 13,
                IT::STMIA => i.rn as u32,
                _ => unreachable!(),
            } as u8;
            // SP subtraction
            vec.push(I {
                it: IT::SUBSP,
                rd: target,
                immu: n << 2,
                imms: 0,
                rm: 0,
                rn: 0,
                rt: 0,
                rl: 0,
                setflags: false,
            });
            let mut sp_offset = 0;
            for r in 0..15 {
                if bit_as_bool(i.rl as u32, r) {
                    vec.push(I {
                        it: IT::STRImm,
                        rt: r as u8,
                        immu: sp_offset,
                        imms: 0,
                        rn: target,
                        rd: 0,
                        rl: 0,
                        rm: 0,
                        setflags: false
                    });
                    sp_offset += 4
                }
            }
        }
        IT::POP | IT::LDMIA => {
            let n = hamming_weight(i.rl as u32);
            let target = match i.it {
                IT::POP => 13,
                IT::STMIA => i.rn as u32,
                _ => unreachable!(),
            } as u8;
            let mut sp_offset = 0;
            for r in 0..8 {
                if bit_as_bool(i.rl as u32, r) {
                    vec.push(I {
                        it: IT::LDRImm,
                        rt: target,
                        immu: sp_offset,
                        imms: 0,
                        rn: 13,
                        rd: 0,
                        rl: 0,
                        rm: 0,
                        setflags: false
                    });
                    sp_offset += 4
                }
            }
            if bit_as_bool(i.rl as u32, 15) {
                vec.push(I {
                    it: IT::LoadPc,
                    rn: target,
                    rt: 15,
                    immu: sp_offset,
                    imms: 0,
                    rl: 0,
                    rm: 0,
                    rd: 0,
                    setflags: false
                })
            }

            // SP addition
            vec.push(I {
                it: IT::ADDSpImm,
                rd: target,
                immu: n << 2,
                imms: 0,
                rm: 0,
                rn: 0,
                rt: 0,
                rl: 0,
                setflags: false,
            });
        }
        _ => {vec.push(i);},
    }
    vec
}

#[cfg(test)]
mod decode2_test {
    use super::*;

    #[test]
    fn push() {
        let i = I {
            it: IT::PUSH,
            rl: 0b0100000011111111,
            rn: 0,
            rd: 0,
            rt: 0,
            rm: 0,
            immu: 0,
            imms: 0,
            setflags: false,
        };
        println!("{:?}", decode2(i));
    }

    #[test]
    fn pop() {
        let i = I {
            it: IT::POP,
            rl: 0b1000000011111111,
            rn: 0,
            rd: 0,
            rt: 0,
            rm: 0,
            immu: 0,
            imms: 0,
            setflags: false,
        };
        println!("{:?}", decode2(i));
    }
}