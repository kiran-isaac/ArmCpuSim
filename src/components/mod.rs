mod ROB;
mod res_station;

/// The type of instruction, same as IT but the difference is that this does not account
/// for different instruction formats. It just accounts for what needs to be done
/// so all the different forms of add instruction are just ADD, and all the shifts are
/// just LSL or ASL etc
enum IT2 {
    // ALU instructions
    ADD,
    AND,
    OR,
    EOR,
    REV,
    REV16,
    REVSH,
    UXTH,
    UXTB,
    SXTB,
    SXTH,
    LSL,
    LSR,
    ASR,

    // Multiply
    MUL,


    // Load store
    LDB,
    STB,
    LDH,
    STH,
    LDW,
    STW,

    // Branch
    B,
    BL,

    // All of the silly instructions
    SVC,
    DSB,
    DMB,
    ISB,
}