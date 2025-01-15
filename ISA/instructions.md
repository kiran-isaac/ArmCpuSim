32 GP registers
32 FP registers

PC : GP[31]
LR : GP[30]
SP : GP[29]
FP : GP[28]
Flags : GP[25]
Zero : GP[27]
One : GP[26]

Flags: https://community.arm.com/arm-community-blogs/b/architectures-and-processors-blog/posts/condition-codes-1-condition-flags-and-codes
V : overflow (signed overflow)
C : carry (unsigned overflow)
N : negative (set to the sign bit of the result)
Z : zero (set if the result is zero)
Condition codes
EQ (Z = 1)
NE (Z = 0)
CC (C = 0)
CS (C = 1)
MI (N = 1)
PL (N = 0)
LT (unsigned less than: C == 1)
LE (unsigned less than or equal: C == 1 or Z == 1)
GT (unsigned greater than: C == 0 and Z == 0)
GE (unsigned greater than or equal: C == 0 or Z == 1)
ST (signed less than: N != V) 
SE (signed less than or equal: Z == 1 or N != V)
SGT (signed greater than: N == V and Z == 0)
SGE (signed greater than or equal: N == V)


# Instruction Set:
nop

## Load store (20)
| instr                | operation                              | comment | 
|----------------------|----------------------------------------|------------------------------------------------|
| ld ra rb             | GP[ra] = MEM[GP[rb]]                   | |
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

## Int arith 32 bit (12)
| instr                | operation                              | comment |
|----------------------|----------------------------------------|------------------------------------------------|
| and ra rb rc         | GP[ra] = GP[rb] & GP[rc]               | |
| or ra rb rc          | GP[ra] = GP[rb] \| GP[rc]              | |
| xor ra rb rc         | GP[ra] = GP[rb] ^ GP[rc]               | |
| not ra rb            | GP[ra] = ~GP[rb]                       | |
| add ra rb rc         | GP[ra] = GP[rb] + GP[rc]               | |
| sub ra rb rc         | GP[ra] = GP[rb] - GP[rc]               | |
| mul ra rb rc         | GP[ra] = GP[rb] * GP[rc]               | |
| div ra rb rc         | GP[ra] = GP[rb] / GP[rc]               | |
| adds ra rb rc        | GP[ra] = GP[rb] + GP[rc]               | Sets flags |
| subs ra rb rc        | GP[ra] = GP[rb] - GP[rc]               | Sets flags |
| muls ra rb rc        | GP[ra] = GP[rb] * GP[rc]               | Sets flags |
| divs ra rb rc        | GP[ra] = GP[rb] / GP[rc]               | Sets flags |

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
I have 32 instructions, so I need 5 bits for instruction encoding

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
