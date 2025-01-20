// enum InstrType {
//     /// Shift, add, subtract, move and compare
//     SASMC,
//     /// Data Processing
//     DP,
//     /// Special data instructions and branch and exchange
//     BX,
//     /// LDR register or immediate
//     LDR,
//     /// Load store
//     LS,
//     /// Address to Register: PC relative
//     ADRPC,
//     /// Address to Register: SP relative
//     ADDSP,

//     MISC,
//     /// Store Multiple
//     STM,
//     /// Load Multiple
//     LDM,
//     /// Conditional Branch
//     CB,
//     /// B
//     B,
// }

use crate::instructions::bri;

enum InstrType {
    INVALID,
    UNPREDICTABLE,
    /// Add with Carry (register) adds a register value, the carry flag value, and another register value, and writes the result
    /// to the destination register. It updates the condition flags based on the result
    ADC,
    /// This instruction adds an immediate to a register, and writes the result to the destination register, and updates condition flags
    ADDImm,
    /// This instruction adds two register values, and writes the result to the destination register, and perhaps updates condition flags
    ADDReg,
    /// This instruction adds an immediate value to the SP value, and writes the result to the destination register.
    ADDSpImm,
    /// This instruction adds a register value to the SP value, and writes the result to the destination register.
    ADDSpReg,
    /// Address to Register adds an immediate value to the PC value, and writes the result to the destination register.
    ADR,
    /// This instruction performs a bitwise AND of two register values, and writes the result to the destination register. It
    /// updates the condition flags based on the result
    AND,
    /// Arithmetic Shift Right (immediate) shifts a register value right by an immediate number of bits, shifting in copies
    /// of its sign bit, and writes the result to the destination register. It updates the condition flags based on the result.
    ASRImm,
    /// Arithmetic Shift Right (register) shifts a register value right by a variable number of bits, shifting in copies of its
    /// sign bit, and writes the result to the destination register. The variable number of bits is read from the bottom byte of
    /// a register. It updates the condition flags based on the result
    ASRReg,
    /// Branch causes a branch to a target address.
    B,
    /// Bit Clear (register) performs a bitwise AND of a register value and the complement of a register value, and writes
    /// the result to the destination register. It updates the condition flags based on the result.
    BIC,
    /// Breakpoint
    BKPT,
    /// Branch with Link (immediate) calls a subroutine at a PC-relative address.
    BL,
    /// Branch with Link and Exchange calls a subroutine at an address and instruction set specified by a register.
    /// ARMv6-M only supports Thumb execution. An attempt to change the instruction execution state causes an
    /// exception on the instruction at the target address.
    BLX,
    /// Branch and Exchange causes a branch to an address and instruction set specified by a register. ARMv6-M only
    /// supports Thumb execution. An attempt to change the instruction execution state causes an exception on the
    /// instruction at the target address
    BX,
    /// Compare Negative (register) adds two register values. It updates the condition flags based on the result, and discards
    /// the result.
    CMN,
    /// Compare (immediate) subtracts an immediate value from a register value. It updates the condition flags based on
    /// the result, and discards the result.
    CMPImm,
    /// Compare (register) subtracts one register value from another register value. It updates the condition flags based on
    /// the result, and discards the result
    CMPReg,
    /// UNUSED: Change Processor State
    CPS,
    /// UNUSED: Copy is a synonym for move
    CPY,
    /// UNUSED?: Data Memory Barrier acts as a memory barrier. It ensures that all explicit memory accesses that appear in program
    /// order before the DMB instruction are observed before any explicit memory accesses that appear in program order after
    /// the DMB instruction. It does not affect the ordering of any other instructions executing on the processor
    DMB,
    /// Data Synchronization Barrier acts as a special kind of memory barrier. No instruction in program order after this
    /// instruction can execute until this instruction completes. This instruction completes only when both:
    /// - any explicit memory access made before this instruction is complete.
    /// - all cache and branch predictor maintenance operations before this instruction complete.
    DSB,
    /// Exclusive OR (register) performs a bitwise Exclusive OR of two register values, and writes the result to the
    /// destination register. It updates the condition flags based on the result.
    EOR,
    /// Instruction Synchronization Barrier flushes the pipeline in the processor, so that all instructions following the ISB
    /// are fetched from cache or memory after the instruction has completed. It ensures that the effects of context altering
    /// operations, such as those resulting from read or write accesses to the system control space (SCS), that completed
    /// before the ISB instruction are visible to the instructions fetched after the ISB. See Barrier support for system
    /// correctness on page B2-221 for more details.
    ///
    /// In addition, the ISB instruction ensures that any branches that appear in program order after it are always written into
    /// any branch prediction logic with the context that is visible after the ISB instruction. This is required to ensure correct
    /// execution of the instruction stream.
    ISB,
    /// Load Multiple Increment After loads multiple registers from consecutive memory locations using an address from
    /// a base register. The sequential memory locations start at this address, and the address above the last of those
    /// locations is written back to the base register when the base register is not part of the register list.
    LDMIA,
    /// Load Register (immediate) calculates an address from a base register value and an immediate offset, loads a word
    /// from memory, and writes it to a register. Offset addressing is used, see Memory accesses on page A6-97 for more
    /// information.
    LDRImm,
    /// Load Register (literal) calculates an address from the PC value and an immediate offset, loads a word from memory,
    /// and writes it to a register. See Memory accesses on page A6-97 for information about memory accesses
    LDRLit,
    /// Load Register (register) calculates an address from a base register value and an offset register value, loads a word
    /// from memory, and writes it to a register. Offset addressing is used, see Memory accesses on page A6-97 for more
    /// information.
    LDRReg,
    /// Load Register Byte (immediate) calculates an address from a base register value and an immediate offset, loads a
    /// byte from memory, zero-extends it to form a 32-bit word, and writes it to a register. Offset addressing is used, see
    /// Memory accesses on page A6-97 for more information.
    LDRBImm,
    /// Load Register Byte (register) calculates an address from a base register value and an offset register value, loads a
    /// byte from memory, zero-extends it to form a 32-bit word, and writes it to a register. Offset addressing is used, see
    /// Memory accesses on page A6-97 for more information
    LDRBReg,
    /// Load Register Halfword (immediate) calculates an address from a base register value and an immediate offset, loads
    /// a halfword from memory, zero-extends it to form a 32-bit word, and writes it to a register. Offset addressing is used,
    /// see Memory accesses on page A6-97 for more information.
    LDRHImmediate,
    /// Load Register Halfword (register) calculates an address from a base register value and an offset register value, loads
    /// a halfword from memory, zero-extends it to form a 32-bit word, and writes it to a register. Offset addressing is used,
    /// see Memory accesses on page A6-97 for more information.
    LDRHRegister,
    /// Load Register Signed Byte (register) calculates an address from a base register value and an offset register value,
    /// loads a byte from memory, sign-extends it to form a 32-bit word, and writes it to a register. Offset addressing is used,
    /// see Memory accesses on page A6-97 for more information.
    LDRSB,
    /// Load Register Signed Halfword (register) calculates an address from a base register value and an offset register
    /// value, loads a halfword from memory, sign-extends it to form a 32-bit word, and writes it to a register. Offset
    /// addressing is used, see Memory accesses on page A6-97 for more information.
    LDRSH,
    /// Logical Shift Left (immediate) shifts a register value left by an immediate number of bits, shifting in zeros, and
    /// writes the result to the destination register. The condition flags are updated based on the result
    LSLImm,
    /// Logical Shift Left (register) shifts a register value left by a variable number of bits, shifting in zeros, and writes the
    /// result to the destination register. The variable number of bits is read from the bottom byte of a register. The condition
    /// flags are updated based on the result.
    LSLReg,
    /// Logical Shift Right (immediate) shifts a register value right by an immediate number of bits, shifting in zeros, and
    /// writes the result to the destination register. The condition flags are updated based on the result.
    LSRImm,
    /// Logical Shift Right (register) shifts a register value right by a variable number of bits, shifting in zeros, and writes
    /// the result to the destination register. The variable number of bits is read from the bottom byte of a register. The
    /// condition flags are updated based on the result.
    LSRReg,
    /// Move (immediate) writes an immediate value to the destination register. The condition flags are updated based on
    /// the result.
    MOVImm,
    /// Move (register) copies a value from a register to the destination register. Encoding T2 updates the condition flags
    /// based on the value.
    MOVReg,
    /// Move (shifted register) is a pseudo-instruction for ASR, LSL, LSR, and ROR
    MOVShift,
    /// Move to Register from Special register moves the value from the selected special-purpose register into a
    /// general-purpose ARM register.
    MRS,
    /// Move to Special Register from ARM Register moves the value of a general-purpose ARM register to the specified
    /// special-purpose register.
    MSR,
    /// Multiply multiplies two register values. The least significant 32 bits of the result are written to the destination
    /// register. These 32 bits do not depend on whether signed or unsigned calculations are performed.
    MUL,
    /// Bitwise NOT (register) writes the bitwise inverse of a register value to the destination register. The condition flags
    /// are updated based on the result
    MVN,
    /// UNUSED: Negate is a pre-UAL synonym for RSB (immediate) with an immediate value of 0. See RSB (immediate) on
    /// page A6-154 for details
    Neg,
    /// No Operation does nothing. This instruction can be used for code alignment purposes.
    NOP,
    /// Logical OR (register) performs a bitwise, inclusive, OR of two register values, and writes the result to the
    /// destination register. The condition flags are updated based on the result.
    ORR,
    /// Pop Multiple Registers loads a subset, or possibly all, of the general-purpose registers R0-R7 and the PC from the
    /// stack.
    /// If the registers loaded include the PC, the word loaded for the PC is treated as a branch address or an exception
    /// return value and a branch occurs. Bit [0] complies with the ARM architecture interworking rules for branches to
    /// Thumb state execution and must be 1. If bit [0] is 0, a HardFault exception occurs
    POP,
    /// Push Multiple Registers stores a subset, or possibly all, of the general-purpose registers R0-R7 and the LR to the
    /// stack
    PUSH,
    /// Byte-Reverse Word reverses the byte order in a 32-bit register.
    REV,
    /// Byte-Reverse Packed Halfword reverses the byte order in each 16-bit halfword of a 32-bit register.
    REV16,
    /// Byte-Reverse Signed Halfword reverses the byte order in the lower 16-bit halfword of a 32-bit register, and sign
    /// extends the result to 32 bits.
    REVSH,
    /// Rotate Right (register) provides the value of the contents of a register rotated by a variable number of bits. The bits
    /// that are rotated off the right end are inserted into the vacated bit positions on the left. The variable number of bits is
    /// read from the bottom byte of a register. The condition flags are updated based on the result
    ROR,
    /// Reverse Subtract (immediate) subtracts a register value from an immediate value, and writes the result to the
    /// destination register. The condition flags are updated based on the result
    RSB,
    /// Subtract with Carry (register) subtracts a register value and the value of NOT(Carry flag) from a register value, and
    /// writes the result to the destination register. The condition flags are updated based on the result.
    SBC,
    /// Send Event is a hint instruction. It causes an event to be signaled to all CPUs within a multiprocessor system.
    /// This is a NOP-compatible hint, see Hint Instructions on page A6-98.
    SEV,
    /// The Store Multiple Increment After and the Store Multiple Empty Ascending instructions store multiple registers
    /// to consecutive memory locations using an address from a base register. The sequential memory locations start at
    /// this address, and the address above the last of those locations is written back to the base register
    STMIA,
    /// Store Register (immediate) calculates an address from a base register value and an immediate offset, and stores a
    /// word from a register to memory. See Memory accesses on page A6-97 for information about memory accesses
    STRImm,
    /// Store Register (register) calculates an address from a base register value and an offset register value, stores a word
    /// from a register to memory. See Memory accesses on page A6-97 for information about memory accesses
    STRReg,
    /// Store Register Byte (immediate) calculates an address from a base register value and an immediate offset, and stores
    /// a byte from a register to memory. See Memory accesses on page A6-97 for information about memory accesses
    STRBImm,
    /// Store Register Byte (register) calculates an address from a base register value and an offset register value, and stores
    /// a byte from a register to memory. See Memory accesses on page A6-97 for information about memory accesses.
    STRBReg,
    /// Store Register Halfword (immediate) calculates an address from a base register value and an immediate offset, and
    /// stores a halfword from a register to memory. See Memory accesses on page A6-97 for information about memory
    /// accesses.
    STRHImm,
    /// Store Register Halfword (register) calculates an address from a base register value and an offset register value, and
    /// stores a halfword from a register to memory. See Memory accesses on page A6-97 for information about memory
    /// accesses.
    STRHReg,
    /// This instruction subtracts an immediate value from a register value, and writes the result to the destination register.
    /// The condition flags are updated based on the result.
    SUBImm,
    /// This instruction subtracts an optionally-shifted register value from a register value, and writes the result to the
    /// destination register. It updates the condition flags based on the result.
    SUBReg,
    /// This instruction subtracts an immediate value from the SP value, and writes the result to the destination register.
    SUBSP,
    /// The Supervisor Call instruction generates a call to a system supervisor, see Exceptions on page B1-183 for more
    /// information. When the exception is escalated, a HardFault exception is caused.
    /// Use it as a call to an operating system to provide a service
    SVC,
    /// Signed Extend Byte extracts an 8-bit value from a register, sign extends it to 32 bits, and writes the result to the
    /// destination register
    SXTB,
    /// Signed Extend Halfword extracts a 16-bit value from a register, sign extends it to 32 bits, and writes the result to
    /// the destination register
    SXTH,
    /// Test (register) performs a logical AND operation on two register values. It updates the condition flags based on the
    /// result, and discards the result
    TST,
    /// Permanently Undefined generates an Undefined Instruction exception.
    UDF,
    /// Unsigned Extend Byte extracts an 8-bit value from a register, zero extends it to 32 bits, and writes the result to the
    /// destination register.
    UXTB,
    /// Unsigned Extend Halfword extracts a 16-bit value from a register, zero extends it to 32 bits, and writes the result to the
    /// destination register.
    UXTH,
    /// Wait For Event is a hint instruction that permits the processor to enter a low-power state until one of a number of
    /// events occurs, including events signaled by the SEV instruction on any processor in a multiprocessor system. For
    /// more information, see Wait For Event and Send Event on page B1-209.
    WFE,
    /// Wait For Interrupt is a hint instruction that suspends execution until one of a number of events occurs. For more
    /// information, see Wait For Interrupt on page B1-210.
    /// For general hint behavior, see Hint Instructions on page A6-98.
    WFI,
    /// YIELD is a hint instruction. It enables software with a multithreading capability to indicate to the hardware that it is
    /// performing a task, for example a spinlock, that could be swapped out to improve overall system performance.
    /// Hardware can use this hint to suspend and resume multiple code threads if it supports the capability.
    /// For general hint behavior, see Hint Instructions on page A6-98.
    YIELD,
}

struct Instruction {
    it: InstrType,
    rd: u8,
    rn: u8,
    rm: u8,
    imm1: u32,
    imm2: u32,
    setflags: bool,
}

impl Instruction {
    fn invalid() -> Self {
        Instruction {
            it: InstrType::INVALID,
            rd: 0,
            rn: 0,
            rm: 0,
            imm1: 0,
            imm2: 0,
            setflags: false,
        }
    }

    fn unpredictable() -> Self {
        Instruction {
            it: InstrType::UNPREDICTABLE,
            rd: 0,
            rn: 0,
            rm: 0,
            imm1: 0,
            imm2: 0,
            setflags: false,
        }
    }
}

fn decode(i: u32) -> Instruction {
    match bri(i, 31, 16) {
        // Instruction is 16 bit
        0 => {
            match bri(i, 14, 15) {
                // Shift (immediate), add, subtract, move, and compare
                0b00 => {
                    // let opcode = bit_range_inclusive(i, 9, 13);
                    match bri(i, 11, 13) {
                        // LSL or MOV encoding 2
                        0b000 => {
                            let imm5 = bri(i, 6, 10);
                            let rd = bri(i, 0, 2) as u8;
                            let rm = bri(i, 3, 5) as u8;

                            return if imm5 == 0 {
                                Instruction {
                                    it: InstrType::MOVReg,
                                    imm1: 0,
                                    imm2: 0,
                                    rd,
                                    rm,
                                    rn: 0,
                                    setflags: true,
                                }
                            } else {
                                Instruction {
                                    it: InstrType::LSLImm,
                                    imm1: imm5,
                                    imm2: 0,
                                    rd,
                                    rm,
                                    rn: 0,
                                    setflags: true,
                                }
                            };
                        }
                        // LSR
                        0b001 => {
                            let imm5 = bri(i, 6, 10);
                            let rd = bri(i, 0, 2) as u8;
                            let rm = bri(i, 3, 5) as u8;

                            return Instruction {
                                it: InstrType::LSRImm,
                                imm1: imm5,
                                imm2: 0,
                                rd,
                                rm,
                                rn: 0,
                                setflags: true,
                            };
                        }
                        // ASR
                        0b010 => {
                            let imm5 = bri(i, 6, 10);
                            let rd = bri(i, 0, 2) as u8;
                            let rm = bri(i, 3, 5) as u8;

                            return Instruction {
                                it: InstrType::ASRImm,
                                imm1: imm5,
                                imm2: 0,
                                rd,
                                rm,
                                rn: 0,
                                setflags: true,
                            };
                        }
                        // ADDReg (T1), SUBReg (T1), ADDImm (T1), SUBImm (T1)
                        0b011 => {
                            let rmorimm3 = bri(i, 6, 8) as u8;
                            let rn = bri(i, 3, 5) as u8;
                            let rd = bri(i, 0, 2) as u8;

                            return match bri(i, 9, 10) {
                                // ADDReg
                                0b00 => Instruction {
                                    it: InstrType::ADDReg,
                                    rd,
                                    rm: rmorimm3,
                                    rn,
                                    imm1: 0,
                                    imm2: 0,
                                    setflags: true,
                                },
                                0b01 => Instruction {
                                    it: InstrType::SUBReg,
                                    rd,
                                    rm: rmorimm3,
                                    rn,
                                    imm1: 0,
                                    imm2: 0,
                                    setflags: true,
                                },
                                0b10 => Instruction {
                                    it: InstrType::ADDImm,
                                    rd,
                                    rm: 0,
                                    rn,
                                    imm1: rmorimm3 as u32,
                                    imm2: 0,
                                    setflags: true,
                                },
                                0b11 => Instruction {
                                    it: InstrType::SUBImm,
                                    rd,
                                    rm: 0,
                                    rn,
                                    imm1: rmorimm3 as u32,
                                    imm2: 0,
                                    setflags: true,
                                },
                                _ => unreachable!("BRI issue: Invalid instr: {i}"),
                            };
                        }
                        // MOVImm
                        0b100 => {
                            let rd = bri(i, 8, 10) as u8;
                            let imm8 = bri(i, 0, 7);

                            return Instruction {
                                it: InstrType::MOVImm,
                                imm1: imm8,
                                rd,
                                setflags: true,
                                rm: 0,
                                rn: 0,
                                imm2: 0,
                            };
                        }
                        0b101 => {
                            let rn = bri(i, 8, 10) as u8;
                            let imm8 = bri(i, 0, 7);

                            return Instruction {
                                it: InstrType::CMPImm,
                                rn,
                                imm1: imm8,
                                setflags: true,
                                rd: 0,
                                rm: 0,
                                imm2: 0,
                            };
                        }
                        // ADDImm (T2)
                        0b110 => {
                            let rdn = bri(i, 8, 10) as u8;
                            let imm8 = bri(i, 0, 7);

                            return Instruction {
                                it: InstrType::ADDImm,
                                rd: rdn,
                                rn: rdn,
                                imm1: imm8,
                                setflags: true,
                                imm2: 0,
                                rm: 0,
                            };
                        }
                        // SUBImm (T2)
                        0b111 => {
                            let rdn = bri(i, 8, 10) as u8;
                            let imm8 = bri(i, 0, 7);

                            return Instruction {
                                it: InstrType::SUBImm,
                                rd: rdn,
                                rn: rdn,
                                imm1: imm8,
                                setflags: true,
                                imm2: 0,
                                rm: 0,
                            };
                        }
                        _ => unreachable!("BRI issue: Invalid instr: {i}"),
                    }
                }
                // Data Processing
                // Special data instructions, branch and exchange
                // Load from literal pool
                // Loa/store single data item pt1
                0b01 => match bri(i, 10, 13) {
                    // Data Processing
                    0b0000 => {
                        let opcode = bri(i, 6, 9);
                        let rdn = bri(i, 0, 2) as u8;
                        let rm = bri(i, 3, 5) as u8;

                        let it = match opcode {
                            0b0000 => InstrType::AND,
                            0b0001 => InstrType::EOR,
                            0b0010 => InstrType::LSLReg,
                            0b0011 => InstrType::LSRReg,
                            0b0100 => InstrType::ASRReg,
                            0b0101 => InstrType::ADC,
                            0b0110 => InstrType::SBC,
                            0b0111 => InstrType::ROR,
                            0b1000 => InstrType::TST,
                            0b1001 => InstrType::RSB,
                            0b1010 => InstrType::CMPReg, // T1
                            0b1011 => InstrType::CMN,
                            0b1100 => InstrType::ORR,
                            0b1101 => InstrType::MUL,
                            0b1110 => InstrType::BIC,
                            0b1111 => InstrType::MVN,
                            _ => unreachable!("BRI issue: Invalid instr: {i}"),
                        };

                        return Instruction {
                            it,
                            rd: rdn,
                            rn: rdn,
                            rm,
                            imm1: 0,
                            imm2: 0,
                            setflags: true,
                        };
                    }
                    // Special data instructions, branch and exchange
                    0b0001 => match bri(i, 6, 9) {
                        // ADDReg (T2)
                        0b0000 | 0b0001 | 0b0010 | 0b0011 => {
                            let n = bri(i, 7, 7);
                            let rm = bri(i, 3, 6);
                            let rn = bri(i, 0, 2) + n << 4;

                            return Instruction {
                                it: InstrType::CMPReg,
                                rn: rn as u8,
                                rm: rm as u8,
                                setflags: true,
                                rd: 0,
                                imm1: 0,
                                imm2: 0,
                            };
                        }
                        0b0100 => return Instruction::unpredictable(),
                        // CMPReg (T2)
                        0b0101 | 0b0110 | 0b0111 => {
                            let n = bri(i, 7, 7);
                            let rm = bri(i, 3, 6);
                            let rn = bri(i, 0, 2) + n << 4;

                            return Instruction {
                                it: InstrType::CMPReg,
                                rn: rn as u8,
                                rm: rm as u8,
                                setflags: true,
                                rd: 0,
                                imm1: 0,
                                imm2: 0,
                            };
                        },
                        // MOVReg (T1)
                        0b1000 | 0b1001 | 0b1010 | 0b1011 => {
                            let n = bri(i, 7, 7);
                            let rm = bri(i, 3, 6);
                            let rn = bri(i, 0, 2) + n << 4;

                            return Instruction {
                                it: InstrType::MOVReg,
                                rn: rn as u8,
                                rm: rm as u8,
                                setflags: true,
                                rd: 0,
                                imm1: 0,
                                imm2: 0,
                            };
                        }
                        // BX
                        0b1100 | 0b1101 => {
                            let rm = bri(i, 3, 6) as u8;
                            if  bri(i, 0, 2) != 0 {
                                return Instruction::unpredictable()
                            }

                            return Instruction {
                                it: InstrType::BX,
                                rm,
                                setflags: false,
                                rd: 0,
                                rn: 0,
                                imm1: 0,
                                imm2: 0,
                            } 
                        }
                        0b1110 | 0b1111 => {
                            let rm = bri(i, 3, 6) as u8;
                            if  bri(i, 0, 2) != 0 {
                                return Instruction::unpredictable()
                            }

                            return Instruction {
                                it: InstrType::BLX,
                                rm,
                                setflags: false,
                                rd: 0,
                                rn: 0,
                                imm1: 0,
                                imm2: 0,
                            } 
                        }
                        _ => unreachable!("BRI issue: Invalid instr: {i}"),
                    },
                    _ => unreachable!("BRI issue: Invalid instr: {i}"),
                },
                // Load/store single data item pt2
                // PC Relative
                // SP Relative
                // Misc
                0b10 => {}
                // Store Multiple
                // Load Multiple
                // Conditional Branch
                // Unconditional Branch
                0b11 => {}
                _ => panic!("Invalid instr: {i}"),
            }
        }
        // Instruction is 32 bit
        _ => {
            panic!("Invalid instr: {i}")
        }
    }

    panic!("Invalid instr: {i}")
}
