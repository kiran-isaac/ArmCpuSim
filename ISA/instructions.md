16 GP registers
16 FP registers

PC : GP[15]
LR : GP[14]
SP : GP[13]
FP : GP[12]
Flags : GP[11]
Zero : GP[10]
One : GP[9]

Flags: https://community.arm.com/arm-community-blogs/b/architectures-and-processors-blog/posts/condition-codes-1-condition-flags-and-codes
V : overflow (signed overflow)
C : carry (unsigned overflow)
N : negative (set to the sign bit of the result)
Z : zero (set if the result is zero)

# Instruction Set:
## Int arith 32 bit (12)
| instr                | operation                              | comment | encoding |
|----------------------|----------------------------------------|-----------------------------------------|-----|
| and[s] ra rb rc         | GP[ra] = GP[rb] & GP[rc]                | | s <ra:4> <rb:4> <rc:4> |
| or[s] ra rb rc          | GP[ra] = GP[rb] \| GP[rc]               | | s <ra:4> <rb:4> <rc:4> |
| xor[s] ra rb rc         | GP[ra] = GP[rb] ^ GP[rc]                | | s <ra:4> <rb:4> <rc:4> |
| not[s] ra rb            | GP[ra] = ~GP[rb]                        | | s <ra:4> <rb:4> |
| add[s] ra rb rc         | GP[ra] = GP[rb] + GP[rc]                | | s <ra:4> <rb:4> <rc:4> |
| sub[s]ra rb rc         | GP[ra] = GP[rb] - GP[rc]                 | | s <ra:4> <rb:4> <rc:4> |
| mul[s]  ra rb rc         | GP[ra] = GP[rb] * GP[rc]               | | s <ra:4> <rb:4> <rc:4> |
| div[s] ra rb rc         | GP[ra] = GP[rb] / GP[rc]                | | s <ra:4> <rb:4> <rc:4> |

| instr                | operation                              | comment | 
|----------------------|----------------------------------------|------------------------------------------------|
| ld ra rb             | GP[ra] = MEM[GP[rb]]                   | | |
| ldi ra imm           | GP[ra] = imm                           | (load immediate)                              |
| ldo ra rb rc         | GP[ra] = MEM[GP[rb] + GP[rc]]          | (load from address + register offset)         |
| ldio ra rb imm       | GP[ra] = MEM[GP[rb] + imm]             | (load from address + immediate offset)        |
| st ra rb             | MEM[GP[rb]] = GP[ra]                   |                                               |
| sti ra imm           | MEM[ra] = imm                          | (store immediate)                             |
| sto ra rb rc         | MEM[GP[rb] + GP[rc]] = GP[ra]          | (store with register offset)                  |
| stio ra rb imm       | MEM[GP[rb] + imm] = GP[ra]             | (store with immediate offset)                 |
| ldf fa rb            | FP[fa] = MEM[GP[rb]]                   | |
| ldfi fa imm          | FP[fa] = imm                           | (load float immediate)                        |
| ldfo fa rb rc        | FP[fa] = MEM[GP[rb] + GP[rc]]          | (load float from address + register offset)   |
| ldfio fa rb imm      | FP[fa] = MEM[GP[rb] + imm]             | (load float from address + immediate offset)  |
| stf fa rb            | MEM[GP[rb]] = FP[fa]                   | |
| stfi fa imm          | MEM[imm] = FP[fa]                      | (store float immediate)                       |
| stfo fa rb rc        | MEM[GP[rb] + GP[rc]] = FP[fa]          | (store float with register offset)            |
| stfio fa rb imm      | MEM[GP[rb] + imm] = FP[fa]             | (store float with immediate offset)           |
| push ra              | MEM[SP] = GP[ra]; SP = SP - 4          | |
| pop ra               | SP = SP + 4; GP[ra] = MEM[SP]          | |
| pushf fa             | MEM[SP] = FP[fa]; SP = SP - 4          | |
| popf fa              | SP = SP + 4; FP[fa] = MEM[SP]          | |

##  Float arith 32 bit (8)
| instr                | operation                              | comment |
|----------------------|----------------------------------------|------------------------------------------------|
| fadd ra rb rc        | FP[ra] = FP[rb] + FP[rc]               | |
| fsub ra rb rc        | FP[ra] = FP[rb] - FP[rc]               | |
| fmul ra rb rc        | FP[ra] = FP[rb] * FP[rc]               | |
| fdiv ra rb rc        | FP[ra] = FP[rb] / FP[rc]               | |
| fadds ra rb rc       | FP[ra] = FP[rb] + FP[rc]               | Sets flags |
| fsubs ra rb rc       | FP[ra] = FP[rb] - FP[rc]               | Sets flags |
| fmuls ra rb rc       | FP[ra] = FP[rb] * FP[rc]               | Sets flags |
| fdivs ra rb rc       | FP[ra] = FP[rb] / FP[rc]               | Sets flags |

## Control flow (10)
| instr                | operation                              | comment |
|----------------------|----------------------------------------|------------------------------------------------|
| b label              | PC = label                             | |
| j ra                 | PC = GP[ra]                            | |
| beq label            | if (Z == 1) PC = label                 | |
| bne label            | if (Z == 0) PC = label                 | |
| blt label            | if (C == 1) PC = label                 | |
| ble label            | if (C == 1 or Z == 1) PC = label       | |
| bgt label            | if (C == 0 and Z == 0) PC = label      | |
| bge label            | if (C == 0 or Z == 1) PC = label       | |
| call label           | LR = PC + 1; PC = label;               | |
| ret                  | PC = LR                                | | 

## Stack operations (2)
push ra # Decrements SP by 4 and stores GP[ra] at the new SP
pop ra # Loads the value at SP into GP[ra] and increments SP by 4

# Encoding
I have 48 instructions, so I need 6 bits for instruction encoding
Each register takes 5 bits to encode
Each immediate value takes 16 bits to encode
Variable length encoding

## Memory
push    : 000000
pop     : 000001
load    : 000010
loadi   : 000011
loadf   : 000100
loadfi  : 000101
store   : 000110
storef  : 000111

## Control flow
b       : 001000
beq     : 001001
bne     : 001010
blt     : 001011
ble     : 001100
call    : 001101
ret     : 001110
nop     : 001111

# Int arith
add     : 010000
sub     : 010001
mul     : 010010
div     : 010011
adds    : 010100
subs    : 010101
muls    : 010110
divs    : 010111

# Float arith
fadd    : 011000
fsub    : 011001
fmul    : 011010
fdiv    : 011011
fadds   : 011100
fsubs   : 011101
fmuls   : 011110
fdivs   : 011111
