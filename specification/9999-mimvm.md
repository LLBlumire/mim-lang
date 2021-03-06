
# MimVM

MimVM is a virtual machine that evaluates the results of compiling Mim. It is the primary target of
MimC. It executes MimB code. MimC can also compile MimASM to MimB.

The virtual machine uses a small fixed-width instruction format inspired by RISC. Insructions are
64 bis wide.

# Instructions

MimVM instructions are all made of 64 bit components, an instruction can be 1 to 4 components long.

The first component always has the following structure.z

`
| 63 .......... 48 | 47 ............... 32 | 31 ................ 16 | 15 ................ 0 |
| Opcode (16 bits) | First Param (16 bits) | Second Param (16 bits) | Third Param (16 bits) |
`

The Opcode (when combined with the instruction mode context) determines the execution of the instruction, as well as the function of the subsequent three parameters. The following three parameters are either values from $\mathtt{0}_{16}$ to $\mathtt{FFFE}_{16}$.

In the event that one of the subsequent parameters is of value $\mathtt{FFFF}_{16}$ then another component is added to the instruction, this happens in order first, second, third. Only if all three are $\mathtt{FFFF}_{16}$ will the instruction fill all 4 components (all 32 bytes).

## Instruction Mode Context

Opcodes are not all created equal, their are a number of instruction modes, and sub-instruction modes, that change the function of the opcodes.

The three primary instruction modes are

 0. TypeDef
 1. Data
 2. Main

### TypeDef

Contains all the tools for defining the type of data. Primarily referenced by methods for typing information.

It has a number of contexts:

 0. No Context
 1. Frame
 2. Sum
 3. Product

#### No Context Instructions

| Instruction | Function | Opcode | First | Second | Third |
| :- | :- | :- | :- | :- | :- |
| section | Changes top level Instruction Mode | 0 | Section Number | | |
| bytes | A type a certain number of bytes wide. | 1 | Number of Bytes | | |
| buffer | A type of a fat pointer to a contiguous sequence of another type. | 2 | Type of Buffer | | |
| begin_frame | Enter into the Frame Context | FFFC | | | |
| begin_sum | Enter into the Sum Context | FFFD | | | |
| begin_product | Enter into the Product Context | FFFE | | | |

#### Frame Context

| Instruction | Function | Opcode | First | Second | Third |
| :- | :- | :- | :- | :- | :- |
| end_frame | Exit the Frame Context | 0 | | | |
| return | Declares the return output | 1 | Type of Output | | |
| local | Initialize the frame with a type in memory | 2 | Reference Number | Type of Local | |

Locals can be set as inputs.
#### Sum Context

| Instruction | Function | Opcode | First | Second | Third |
| :- | :- | :- | :- | :- | :- |
| end_sum | Exit the Sum Context | 0 | | | |
| variant | Define a variant type | 1 | Variant Tag | Type of Variant | |

#### Product Context

| Instruction | Function | Opcode | First | Second | Third |
| :- | :- | :- | :- | :- | :- |
| end_product | Exit the Product Context | 0 | | | |
| compose | Compose a type in the product | 1 | Field Number | Type of Field | |

### Data

Contains the tools for declaring fixed data sequences.

These are not processed as other instructions, and instead represent a more pure kind of storing constant data. The mnemonics lack opcodes, as their content is written instead. The very first 64 bits represent the length of the data section, which should be immutable.

| Instruction | Function | Opcode | First | Second | Third |
| :- | :- | :- | :- | :- | :- |
| section | Changes top level Instruction Mode | 0 | Section Number	| | |
| number | A single number | - | base | bytes | number |
| signed | A signed number | - | base | bytes | number |
| string | A sequence of characters | - | `Character sequence... | ... | ...' |

Strings are stored as a pure sequence. It is up to the programmer to specify a length or ending null byte. They begin reading at a bactick, and continue until an apostrophe, and will copy the raw bytes from the file.


### Main

Contains the main body of the program. It implicitl defines a single local slot typed `bytes 8`, 0, for the return status code.

It has a number of contexts.

 0. No Context
 1. Frame Body

#### No Context Instructions

| Instruction | Function | Opcode | First | Second | Third |
| :- | :- | :- | :- | :- | :- |
| section | Changes top level Instruction Mode | 0 | Section Number | | |
| begin_frame_body | Begins a frame body | 1 | Frame Type | Return local | |

#### Frame Body

| Instruction | Function | Opcode | First | Second | Third |
| :- | :- | :- | :- | :- | :- |
| end_frame_body | Ends a frame body | 0 | Return local | | |
| init_bytes_data | Initialise bytes | 1 | Local Bytes | Data | |
| init_buffer_data | Initialise buffer | 2 | Local Buffer | Data | Length Local |
| copy_bytes_local | Copy bytes from a local | 3 | Local Bytes | Local Copy |
| copy_buffer_local | Copy a buffer from a local | 4 | Local Buffer | Local Copy | Length Local |
| copy_product_local | Copy a product from a local | 5 | Local Product | Local Copy | |
| copy_sum_local | Copy a sum from a local | 6 | Local Sum | Local Copy | |
| set_product_local | Set composed product | 7 | Local Product | Field Number | Local Copy |
| set_sum_local | Set sum variant | 8 | Local Sum | Variant Tag | Local Copy |
| set_frame_local | Sets a frame domain, cleared after call_frame | 9 |  Frame | Reference Number | Local |
| call_frame | Jumps to a frame, returning after execution | A | Frame | Return local |
| add | Adds numbers; `C=A+B`| B | Local A | Local B | Local C |
| sadd | Adds signed numbers; `C=A+B`| C | Local A | Local B | Local C |
| sub | Subtracts numbers; `C=A-B`| D | Local A | Local B | Local C |
| ssub | Subtracts signed numbers; `C=A-B`| E | Local A | Local B | Local C |
| mul | Multiplies numbers; `C=A*B`| F | Local A | Local B | Local C |
| smul | Multiplies signed numbers; `C=A*B`| 10 | Local A | Local B | Local C |
| div | Divides numbers; `C=A/B`| 11 | Local A | Local B | Local C |
| sdiv | Divides signed numbers; `C=A/B`| 12 | Local A | Local B | Local C |
| rem | Remainders numbers; `C=A%B`| 13 | Local A | Local B | Local C |
| srem | Remainders signed numbers; `C=A%B` | 14 | Local A | Local B | Local C |
| fadd | Adds floats; `C=A+B` | 15 | Local A | Local B | Local C |
| fsub | Subtracts floats; `C=A-B` | 16 | Local A | Local B | Local C |
| fmul | Multiplies floats; `C=A*B` | 17 | Local A | Local B | Local C |
| fdiv | Divides floats; `C=A*B` | 18 | Local A | Local B | Local C |
| and | Bitwise and; `C=A*B` | 19 | Local A | Local B | Local C |
| or | Bitwise or; `C=A|B` | 1A | Local A | Local B | Local C |
| xor | Bitwise xor; `C=A^B` | 1B | Local A | Local B | Local C |
| not | Bitwise not; `B=!A` | 1C | Local A | Local B | |
| shl | Bitwise left shift `C=A<<B` | 1D | Local A | Local B | Local C |
| shr | Bitwise right shift `C=A>>B` | 1E | Local A | Local B | Local C |
| ashr | Arithmetic right shift `C=A>>B` | 1F | Local A | Local B | Local C |
| rll | Bitwise left roll `C=A<<B` | 20 | Local A | Local B | Local C |
| rlr | Bitwise right roll `C=A>>B` | 21 | Local A | Local B | Local C |
| bpush | Copy append to end of buffer | 22 | Local Buffer | Local Copy |
| bpop | Copy remove from end of buffer | 23 | Local Buffer | Local Copy |
| bget | Copy get from buffer | 24 | Local Buffer | Index | Local Copy | 
| bput | Copy write to buffer | 25 | Local Buffer | Index | Local Copy | 
| blen | Length of buffer | 26 | Local Buffer | | |
| eq | Check equality of bytes, `C=(A==B)` | 27 | Local A | Local B | Local C |
| leq | Check ordering of numbers `C=(A<=B)` | 28 | Local A | Local B | Local C |
| sleq | Check ordering of signed numbers `C=(A<=B)` | 29 | Local A | Local B | Local C |
| fleq | Check ordering of floats `C=(A<=B)` | 2A | Local A | Local B | Local C |
| geq | Check ordering of numbers `C=(A>=B)` | 2B | Local A | Local B | Local C |
| sgeq | Check ordering of signed numbers `C=(A>=B)` | 2C | Local A | Local B | Local C |
| fgeq | Check ordering of floats `C=(A>=B)` | 2D | Local A | Local B | Local C |
| jump | Jumps execution to an address | 2E | Address | | |
| jumpif | Jummps execution to an address if true | 2F | Address | Condition | |
| stdout_string | Output a string to the standard output | FFFC | Local | Length Local |
| stdout_dec | Output a decimal number to the standard output | FFFE | Local | |