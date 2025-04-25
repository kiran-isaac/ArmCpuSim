use crate::binary::{bit_as_bool, briz};

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IT {
    UNPREDICTABLE,
    UNDEFINED,

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
    // ADDSpReg,
    /// Address to Register adds an immediate value to the PC value, and writes the result to the destination register.
    // ADR, Encoded by ADDReg
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
    // CPS,
    /// UNUSED: Copy is a synonym for move
    // CPY,
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
    // LDRLit, encoded by LDRImm
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
    LDRHImm,
    /// Load Register Halfword (register) calculates an address from a base register value and an offset register value, loads
    /// a halfword from memory, zero-extends it to form a 32-bit word, and writes it to a register. Offset addressing is used,
    /// see Memory accesses on page A6-97 for more information.
    LDRHReg,
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
    // SUBSP, // encoded by subimm
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

    /// Not a real architectural instruction. This is to ensure that loads to PC are treated specially.
    /// Rn is addr
    /// immu is offset
    SetPC,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct I {
    pub it: IT,
    pub rd: u8,
    /// rn or cond or option depending on instr
    pub rn: u8,
    pub rm: u8,
    pub rt: u8,
    pub rl: u16,
    pub immu: u32,
    pub imms: i32,
    pub setsflags: bool,
}

impl I {
    fn unpredictable() -> Self {
        I {
            it: IT::UNPREDICTABLE,
            rd: 0,
            rn: 0,
            rm: 0,
            rt: 0,
            rl: 0,
            immu: 0,
            imms: 0,
            setsflags: false,
        }
    }

    pub fn undefined() -> Self {
        I {
            it: IT::UNDEFINED,
            rd: 0,
            rn: 0,
            rm: 0,
            rt: 0,
            rl: 0,
            immu: 0,
            imms: 0,
            setsflags: false,
        }
    }
}

pub fn decode(i: u32) -> I {
    match briz(i, 16, 31) {
        // Instruction is 16 bit
        0 => {
            match briz(i, 14, 15) {
                // Shift (immediate), add, subtract, move, and compare
                0b00 => {
                    // let opcode = bit_range_inclusive(i, 9, 13);
                    match briz(i, 11, 13) {
                        // LSL or MOVReg (T2)
                        0b000 => {
                            let imm5 = briz(i, 6, 10);
                            let rd = briz(i, 0, 2) as u8;
                            let rm = briz(i, 3, 5) as u8;

                            if imm5 == 0 {
                                I {
                                    it: IT::MOVReg,
                                    immu: 0,
                                    imms: 0,
                                    rd,
                                    rm,
                                    rn: 0,
                                    rt: 0,
                                    rl: 0,
                                    setsflags: true,
                                }
                            } else {
                                I {
                                    it: IT::LSLImm,
                                    immu: imm5,
                                    imms: 0,
                                    rd,
                                    rm,
                                    rn: 0,
                                    rt: 0,
                                    rl: 0,
                                    setsflags: true,
                                }
                            }
                        }
                        // LSR
                        0b001 => {
                            let imm5 = briz(i, 6, 10);
                            let imm5 = if imm5 == 0 { 32 } else { imm5 };
                            let rd = briz(i, 0, 2) as u8;
                            let rm = briz(i, 3, 5) as u8;

                            I {
                                it: IT::LSRImm,
                                immu: imm5,
                                imms: 0,
                                rd,
                                rm,
                                rn: 0,
                                rt: 0,
                                rl: 0,
                                setsflags: true,
                            }
                        }
                        // ASR
                        0b010 => {
                            let imm5 = briz(i, 6, 10);
                            let imm5 = if imm5 == 0 { 32 } else { imm5 };
                            let rd = briz(i, 0, 2) as u8;
                            let rm = briz(i, 3, 5) as u8;

                            return I {
                                it: IT::ASRImm,
                                immu: imm5,
                                imms: 0,
                                rd,
                                rm,
                                rn: 0,
                                rt: 0,
                                rl: 0,
                                setsflags: true,
                            };
                        }
                        // ADDReg (T1), SUBReg (T1), ADDImm (T1), SUBImm (T1)
                        0b011 => {
                            let rmorimm3 = briz(i, 6, 8) as u8;
                            let rn = briz(i, 3, 5) as u8;
                            let rd = briz(i, 0, 2) as u8;

                            return match briz(i, 9, 10) {
                                // ADDReg
                                0b00 => I {
                                    it: IT::ADDReg,
                                    rd,
                                    rm: rmorimm3,
                                    rn,
                                    rt: 0,
                                    immu: 0,
                                    imms: 0,
                                    rl: 0,
                                    setsflags: true,
                                },
                                0b01 => I {
                                    it: IT::SUBReg,
                                    rd,
                                    rm: rmorimm3,
                                    rn,
                                    rt: 0,
                                    immu: 0,
                                    imms: 0,
                                    rl: 0,
                                    setsflags: true,
                                },
                                0b10 => I {
                                    it: IT::ADDImm,
                                    rd,
                                    rn,
                                    rm: 0,
                                    rt: 0,
                                    immu: rmorimm3 as u32,
                                    imms: 0,
                                    rl: 0,
                                    setsflags: true,
                                },
                                0b11 => I {
                                    it: IT::SUBImm,
                                    rd,
                                    rn,
                                    rm: 0,
                                    rt: 0,
                                    immu: rmorimm3 as u32,
                                    imms: 0,
                                    rl: 0,
                                    setsflags: true,
                                },
                                _ => unreachable!("BRI issue: Invalid instr: {i}"),
                            };
                        }
                        // MOVImm
                        0b100 => {
                            let rd = briz(i, 8, 10) as u8;
                            let imm8 = briz(i, 0, 7);

                            I {
                                it: IT::MOVImm,
                                immu: imm8,
                                rd,
                                setsflags: true,
                                rm: 0,
                                rn: 0,
                                rl: 0,
                                rt: 0,
                                imms: 0,
                            }
                        }
                        0b101 => {
                            let rn = briz(i, 8, 10) as u8;
                            let imm8 = briz(i, 0, 7);

                            I {
                                it: IT::CMPImm,
                                rn,
                                immu: imm8,
                                setsflags: true,
                                rd: 0,
                                rm: 0,
                                rt: 0,
                                rl: 0,
                                imms: 0,
                            }
                        }
                        // ADDImm (T2)
                        0b110 => {
                            let rdn = briz(i, 8, 10) as u8;
                            let imm8 = briz(i, 0, 7);

                            I {
                                it: IT::ADDImm,
                                rd: rdn,
                                rn: rdn,
                                immu: imm8,
                                setsflags: true,
                                imms: 0,
                                rm: 0,
                                rt: 0,
                                rl: 0,
                            }
                        }
                        // SUBImm (T2)
                        0b111 => {
                            let rdn = briz(i, 8, 10) as u8;
                            let imm8 = briz(i, 0, 7);

                            I {
                                it: IT::SUBImm,
                                rd: rdn,
                                rn: rdn,
                                immu: imm8,
                                setsflags: true,
                                imms: 0,
                                rm: 0,
                                rt: 0,
                                rl: 0,
                            }
                        }
                        _ => unreachable!("BRI issue: Invalid instr: {i}"),
                    }
                }
                // Data Processing
                // Special data instructions, branch and exchange
                // Load from literal pool
                // Load/store single data item register
                // Load/store single data item immediate
                0b01 => match briz(i, 10, 13) {
                    // Data Processing
                    0b0000 => {
                        let opcode = briz(i, 6, 9);
                        let lowreg = briz(i, 0, 2) as u8;
                        let highreg = briz(i, 3, 5) as u8;

                        let it = match opcode {
                            0b0000 => IT::AND,
                            0b0001 => IT::EOR,
                            0b0010 => IT::LSLReg,
                            0b0011 => IT::LSRReg,
                            0b0100 => IT::ASRReg,
                            0b0101 => IT::ADC,
                            0b0110 => IT::SBC,
                            0b0111 => IT::ROR,
                            0b1000 => IT::TST,
                            // An annoying special case
                            0b1001 => {
                                return I {
                                    it: IT::RSB,
                                    rd: lowreg,
                                    rn: highreg,
                                    rm: 0,
                                    immu: 0,
                                    imms: 0,
                                    rt: 0,
                                    rl: 0,
                                    setsflags: true,
                                }
                            }
                            0b1010 => IT::CMPReg, // T1
                            0b1011 => IT::CMN,
                            0b1100 => IT::ORR,
                            0b1101 => {
                                return I {
                                    it: IT::MUL,
                                    rd: lowreg,
                                    rn: highreg,
                                    rm: lowreg,
                                    immu: 0,
                                    imms: 0,
                                    rt: 0,
                                    rl: 0,
                                    setsflags: true,
                                }
                            }
                            0b1110 => IT::BIC,
                            0b1111 => IT::MVN,
                            _ => unreachable!("BRI issue: Invalid instr: {i}"),
                        };

                        I {
                            it,
                            rd: lowreg,
                            rn: lowreg,
                            rm: highreg,
                            immu: 0,
                            imms: 0,
                            rt: 0,
                            rl: 0,
                            setsflags: true,
                        }
                    }
                    // Special data instructions, branch and exchange
                    0b0001 => match briz(i, 6, 9) {
                        // ADDReg (T2)
                        0b0000 | 0b0001 | 0b0010 | 0b0011 => {
                            let n = briz(i, 7, 7);
                            let rm = briz(i, 3, 6);
                            let rdn = briz(i, 0, 2) + (n << 3);

                            I {
                                it: IT::ADDReg,
                                rn: rdn as u8,
                                rm: rm as u8,
                                setsflags: true,
                                rd: rdn as u8,
                                immu: 0,
                                imms: 0,
                                rt: 0,
                                rl: 0,
                            }
                        }
                        0b0100 => return I::unpredictable(),
                        // CMPReg (T2)
                        0b0101 | 0b0110 | 0b0111 => {
                            let n = briz(i, 7, 7);
                            let rm = briz(i, 3, 6);
                            let rn = briz(i, 0, 2) + (n << 3);

                            I {
                                it: IT::CMPReg,
                                rn: rn as u8,
                                rm: rm as u8,
                                setsflags: true,
                                rd: 0,
                                rt: 0,
                                rl: 0,
                                immu: 0,
                                imms: 0,
                            }
                        }
                        // MOVReg (T1)
                        0b1000..=0b1011 => {
                            let d = briz(i, 7, 7);
                            let rm = briz(i, 3, 6);
                            let rd = briz(i, 0, 2);
                            let rd = rd + (d << 3);

                            I {
                                it: IT::MOVReg,
                                rd: rd as u8,
                                rm: rm as u8,
                                setsflags: true,
                                rn: 0,
                                rt: 0,
                                rl: 0,
                                immu: 0,
                                imms: 0,
                            }
                        }
                        // BX
                        0b1100 | 0b1101 => {
                            let rm = briz(i, 3, 6) as u8;
                            if briz(i, 0, 2) != 0 {
                                return I::unpredictable();
                            }

                            I {
                                it: IT::BX,
                                rm,
                                setsflags: false,
                                rd: 0,
                                rt: 0,
                                rn: 0,
                                rl: 0,
                                immu: 0,
                                imms: 0,
                            }
                        }
                        0b1110 | 0b1111 => {
                            let rm = briz(i, 3, 6) as u8;
                            if briz(i, 0, 2) != 0 {
                                return I::unpredictable();
                            }

                            I {
                                it: IT::BLX,
                                rm,
                                setsflags: false,
                                rd: 0,
                                rn: 0,
                                immu: 0,
                                imms: 0,
                                rt: 0,
                                rl: 0,
                            }
                        }
                        _ => unreachable!("BRI issue: Invalid instr: {i}"),
                    },
                    // Load from literal pool
                    0b0010 | 0b0011 => {
                        let rt = briz(i, 8, 10) as u8;
                        let imm8 = briz(i, 0, 7) << 2;

                        return I {
                            it: IT::LDRImm,
                            rt,
                            immu: imm8,
                            imms: 0,
                            rd: 0,
                            rn: 15,
                            rm: 0,
                            rl: 0,
                            setsflags: false,
                        };
                    }
                    // Load/store single data item register
                    0b0100 | 0b0101 | 0b0110 | 0b0111 => {
                        let rt = briz(i, 0, 2) as u8;
                        let rn = briz(i, 3, 5) as u8;
                        let rm = briz(i, 6, 8) as u8;

                        let it = match briz(i, 9, 11) {
                            0b000 => IT::STRReg,
                            0b001 => IT::STRHReg,
                            0b010 => IT::STRBReg,
                            0b011 => IT::LDRSB,
                            0b100 => IT::LDRReg,
                            0b101 => IT::LDRHReg,
                            0b110 => IT::LDRBReg,
                            0b111 => IT::LDRSH,
                            _ => unreachable!("BRI issue: Invalid instr: {i}"),
                        };

                        return I {
                            it,
                            rt,
                            rn,
                            rm,
                            rd: 0,
                            rl: 0,
                            immu: 0,
                            imms: 0,
                            setsflags: false,
                        };
                    }
                    // Load/store single data item immediate
                    0b1000..=0b1111 => {
                        let imm5 = briz(i, 6, 10);
                        let rn = briz(i, 3, 5) as u8;
                        let rt = briz(i, 0, 2) as u8;

                        let it = match briz(i, 11, 12) {
                            // STRImm (T1)
                            0b00 => IT::STRImm,
                            // LDRImm (T1)
                            0b01 => IT::LDRImm,

                            0b10 => IT::STRBImm,
                            0b11 => IT::LDRBImm,
                            _ => unreachable!("BRI issue: Invalid instr: {i}"),
                        };

                        let immu = match it {
                            IT::STRImm | IT::LDRImm => imm5 << 2,
                            _ => imm5,
                        };

                        I {
                            it,
                            rn,
                            rt,
                            rm: 0,
                            rd: 0,
                            rl: 0,
                            immu,
                            imms: 0,
                            setsflags: false,
                        }
                    }
                    _ => unreachable!("BRI issue: Invalid instr: {i}"),
                },
                // Load/store single data item pt2
                // ADR: PC Relative
                // ADDSpImm: SP Relative
                // Misc
                0b10 => match briz(i, 10, 13) {
                    // Load/store single data item pt2
                    0b0000..=0b0111 => {
                        let it = match briz(i, 11, 12) {
                            0b00 => IT::STRHImm,
                            0b01 => IT::LDRHImm,
                            0b10 => IT::STRImm,
                            0b11 => IT::LDRImm,
                            _ => unreachable!("BRI issue: Invalid instr: {i}"),
                        };

                        let (immu, rn, rt) = match it {
                            IT::STRImm | IT::LDRImm => {
                                let imm8 = briz(i, 0, 7);
                                let rt = briz(i, 8, 10) as u8;
                                let rn = 13;

                                (imm8 << 2, rn, rt)
                            }
                            IT::STRHImm | IT::LDRHImm => {
                                let imm5 = briz(i, 6, 10);
                                let rn = briz(i, 3, 5) as u8;
                                let rt = briz(i, 0, 2) as u8;

                                (imm5 << 1, rn, rt)
                            }
                            _ => unreachable!("BRI issue: Invalid instr: {i}"),
                        };

                        I {
                            it,
                            rn,
                            rt,
                            immu,
                            rm: 0,
                            rd: 0,
                            rl: 0,
                            imms: 0,
                            setsflags: false,
                        }
                    }
                    // ADR
                    0b1000 | 0b1001 => {
                        let rd = briz(i, 8, 10) as u8;
                        let imm8 = briz(i, 0, 7);

                        I {
                            it: IT::ADDImm,
                            rd,
                            immu: imm8,
                            imms: 0,
                            rm: 0,
                            rn: 15,
                            rt: 0,
                            rl: 0,
                            setsflags: false,
                        }
                    }
                    // ADDSpImm (T1)
                    0b1010 | 0b1011 => {
                        let rd = briz(i, 8, 10) as u8;
                        let imm8 = briz(i, 0, 7) << 2;

                        I {
                            it: IT::ADDImm,
                            rd,
                            immu: imm8,
                            imms: 0,
                            rm: 0,
                            rn: 13,
                            rt: 0,
                            rl: 0,
                            setsflags: false,
                        }
                    }
                    // Misc
                    0b1100..=0b1111 => {
                        match briz(i, 5, 11) {
                            // ADDSpImm (T2)
                            0b0000000..=0b0000011 => {
                                let imm7 = briz(i, 0, 6) << 2;

                                I {
                                    it: IT::ADDImm,
                                    rd: 13,
                                    immu: imm7,
                                    imms: 0,
                                    rm: 0,
                                    rn: 13,
                                    rt: 0,
                                    rl: 0,
                                    setsflags: false,
                                }
                            }
                            // SUBSP
                            0b0000100..=0b0000111 => {
                                let imm7 = briz(i, 0, 6) << 2;

                                I {
                                    it: IT::SUBImm,
                                    rd: 13,
                                    immu: imm7,
                                    imms: 0,
                                    rm: 0,
                                    rn: 13,
                                    rt: 0,
                                    rl: 0,
                                    setsflags: false,
                                }
                            }
                            // SXTH, SXTB, UXTH, UXTB
                            0b0010000..=0b0010111 => {
                                let rd = briz(i, 0, 2) as u8;
                                let rm = briz(i, 3, 5) as u8;

                                let it = match briz(i, 6, 7) {
                                    0b00 => IT::SXTH,
                                    0b01 => IT::SXTB,
                                    0b10 => IT::UXTH,
                                    0b11 => IT::UXTB,
                                    _ => unreachable!("BRI issue: Invalid instr: {i}"),
                                };

                                I {
                                    it,
                                    rd,
                                    rm,
                                    rn: 0,
                                    rt: 0,
                                    rl: 0,
                                    setsflags: false,
                                    immu: 0,
                                    imms: 0,
                                }
                            }
                            // Push multiple registers
                            0b0100000..=0b0101111 => {
                                let m = briz(i, 8, 8);
                                let rl = briz(i, 0, 7) + (m << 14);

                                I {
                                    it: IT::PUSH,
                                    rl: rl as u16,
                                    rn: 0,
                                    rd: 0,
                                    rt: 0,
                                    rm: 0,
                                    immu: 0,
                                    imms: 0,
                                    setsflags: false,
                                }
                            }
                            0b0110011 => unimplemented!("CPS"),
                            0b1010000..=0b1010111 => {
                                let rm = briz(i, 3, 5) as u8;
                                let rd = briz(i, 0, 2) as u8;

                                let it = match briz(i, 6, 7) {
                                    0b00 => IT::REV,
                                    0b01 => IT::REV16,
                                    0b10 => panic!("Invalid Instruction: {i}"),
                                    0b11 => IT::REVSH,
                                    _ => unreachable!("BRI issue: Invalid instr: {i}"),
                                };

                                I {
                                    it,
                                    rm,
                                    rd,
                                    rn: 0,
                                    rt: 0,
                                    rl: 0,
                                    immu: 0,
                                    imms: 0,
                                    setsflags: false,
                                }
                            }
                            // Pop
                            0b1100000..=0b1101111 => {
                                let p = briz(i, 8, 8);
                                let rl = briz(i, 0, 7) + (p << 15);

                                return I {
                                    it: IT::POP,
                                    rl: rl as u16,
                                    rn: 0,
                                    rd: 0,
                                    rt: 0,
                                    rm: 0,
                                    immu: 0,
                                    imms: 0,
                                    setsflags: false,
                                };
                            }
                            // BKPT
                            0b1110000..=0b1110111 => {
                                let imm8 = briz(i, 0, 7);

                                return I {
                                    it: IT::BKPT,
                                    immu: imm8,
                                    rd: 0,
                                    rn: 0,
                                    rm: 0,
                                    rt: 0,
                                    rl: 0,
                                    imms: 0,
                                    setsflags: false,
                                };
                            }
                            0b1111000..=0b1111111 => {
                                let opa = briz(i, 4, 7);
                                let opb = briz(i, 0, 3);
                                match (opa, opb) {
                                    (0, 0) => I {
                                        it: IT::NOP,
                                        imms: 0,
                                        immu: 0,
                                        rd: 0,
                                        rl: 0,
                                        rm: 0,
                                        rt: 0,
                                        rn: 0,
                                        setsflags: false,
                                    },
                                    _ => unimplemented!("Hints other than NOP"),
                                }
                            }
                            _ => unreachable!("BRI issue: Invalid instr: {i}"),
                        }
                    }
                    _ => unreachable!("BRI issue: Invalid instr: {i}"),
                },
                // Store Multiple
                // Load Multiple
                // Conditional Branch
                // Unconditional Branch
                0b11 => match briz(i, 10, 13) {
                    // STM
                    0b0000 | 0b0001 => {
                        let rn = briz(i, 8, 10) as u8;
                        let rl = briz(i, 0, 7) as u16;

                        return I {
                            it: IT::STMIA,
                            rl,
                            rn,
                            rd: 0,
                            rm: 0,
                            rt: 0,
                            immu: 0,
                            imms: 0,
                            setsflags: false,
                        };
                    }
                    // LDM
                    0b0010 | 0b0011 => {
                        let rn = briz(i, 8, 10) as u8;
                        let rl = briz(i, 0, 7) as u16;

                        return I {
                            it: IT::LDMIA,
                            rl,
                            rn,
                            rd: 0,
                            rm: 0,
                            rt: 0,
                            immu: 0,
                            imms: 0,
                            setsflags: false,
                        };
                    }
                    // Conditional Branch, UDF and SVC
                    0b0100..=0b0111 => {
                        match briz(i, 8, 11) {
                            0b1110 => return I::undefined(),
                            // SVC and B (T1)
                            _ => {
                                let cond = briz(i, 8, 11);

                                match cond {
                                    0b0000..=0b1110 => {
                                        decode_b1(i)
                                    }
                                    0b1111 => I {
                                        it: IT::SVC,
                                        immu: briz(i, 0, 7),
                                        rn: 0,
                                        rd: 0,
                                        rt: 0,
                                        rl: 0,
                                        rm: 0,
                                        imms: 0,
                                        setsflags: false,
                                    },
                                    _ => unreachable!("BRI issue: Invalid instr: {i}"),
                                }
                            }
                        }
                    }
                    // B (T2)
                    0b1000 | 0b1001 => {
                        decode_b2(i)
                    }
                    _ => panic!("Invalid instr: {i}"),
                },
                _ => panic!("Invalid instr: {i}"),
            }
        }
        // Instruction is 32 bit
        _ => {
            if briz(i, 29, 31) != 0b111 {
                panic!("Invalid instr: {i}")
            }

            let op1 = briz(i, 27, 28);
            let op = briz(i, 15, 15);

            match (op1, op) {
                (0b01, _) | (0b11, _) | (0b10, 0) => panic!("Undefined instr: {i}"),
                (0b10, 1) => {
                    let op1 = briz(i, 20, 26);
                    let op2 = briz(i, 12, 14);

                    match (op2, op1) {
                        // MSR
                        (0b000, 0b0111000)
                        | (0b000, 0b0111001)
                        | (0b010, 0b0111000)
                        | (0b010, 0b0111001) => {
                            unimplemented!()
                        }
                        // MRS
                        (0b000, 0b0111110)
                        | (0b000, 0b0111111)
                        | (0b010, 0b0111110)
                        | (0b010, 0b0111111) => {
                            unimplemented!()
                        }
                        // UDF
                        (0b010, 0b1111111) => I::undefined(),
                        // Misc control instructions
                        (0b000, 0b0111011) | (0b010, 0b0111011) => {
                            let option = briz(i, 0, 3);
                            let it = match briz(i, 4, 7) {
                                0b0100 => IT::DSB,
                                0b0101 => IT::DMB,
                                0b0110 => IT::ISB,
                                _ => panic!("Invalid instr: {i}"),
                            };

                            I {
                                it,
                                rd: 0,
                                rn: option as u8,
                                rm: 0,
                                rt: 0,
                                rl: 0,
                                immu: 0,
                                imms: 0,
                                setsflags: false,
                            }
                        }
                        // BL
                        (0b101, _) | (0b111, _) => {
                            decode_bl(i)
                        }
                        _ => panic!("Invalid instr: {i}"),
                    }
                }
                _ => panic!("Invalid instr: {i}"),
            }
        }
    }
}


pub fn decode_bl(i: u32) -> I {
    let s = bit_as_bool(i, 26);
    let j1 = bit_as_bool(i, 13);
    let j2 = bit_as_bool(i, 11);

    let i1 = !(j1 ^ s) as u32;
    let i2 = !(j2 ^ s) as u32;

    let imm11 = briz(i, 0, 10);
    let imm10 = briz(i, 16, 25);

    let s = if s { 0b1111_1111 } else { 0 };

    let imm32 = i32::from_ne_bytes(
        ((imm11 << 1)
            + (imm10 << 12)
            + (i2 << 22)
            + (i1 << 23)
            + (s << 24))
            .to_ne_bytes(),
    );

    I {
        it: IT::BL,
        imms: imm32,
        rd: 0,
        rn: 0,
        rm: 0,
        rt: 0,
        rl: 0,
        immu: 0,
        setsflags: false,
    }
}

pub fn decode_b1(i: u32) -> I {
    let imm7 = briz(i, 0, 6) << 1;
    let sign = bit_as_bool(i, 7);

    let imm8 = i32::from_ne_bytes(
        (imm7
            + if sign {
            0b1111_1111_1111_1111_1111_1111 << 8
        } else {
            0
        })
            .to_ne_bytes(),
    );

    I {
        it: IT::B,
        imms: imm8,
        rn: briz(i, 8, 11) as u8,
        rd: 0,
        rt: 0,
        rl: 0,
        rm: 0,
        immu: 0,
        setsflags: false,
    }
}

pub fn decode_b2(i: u32) -> I {
    let imm10 = briz(i, 0, 9) << 1;
    let sign = bit_as_bool(i, 10);

    let imm11 = i32::from_ne_bytes(
        (imm10
            + if sign {
            0b1111_1111_1111_1111_1111_11 << 11
        } else {
            0
        })
            .to_ne_bytes(),
    );

    I {
        it: IT::B,
        imms: imm11,
        rn: 0b1110,
        rd: 0,
        rt: 0,
        rl: 0,
        rm: 0,
        immu: 0,
        setsflags: false,
    }
}