# MimVM

MimVM is a virtual machine that evaluates the results of compiling Mim. It is the primary target of
MimC. It executes MimB code. MimC can also compile MimASM to MimB.

The virtual machine uses a small fixed-width instruction format inspired by RISC. Insructions are
64 bis wide.

MimVM has 5 main components. The 'program', which is te loaded sequence of MimVM instructions 
being executed. The 'special registers' which track program mutable registers that have specific 
functions. The 'general registers' which are used for general computaion. The 'stack' which keeps
sequential data pertinent to the current execution. `internal constants` which are read only or
write once information stored by the compiler.

All of these components are virtalised, and so will not match up with their hardware equivalents.

It is a high level virtual machine, that executes a high level machine code. Although direct sysem
calls are possible, usually `!methods` (bang-methods) are used to interface with the machine. These
are special instructions understood by the virtual machine that perform higher level functions.

## Special Registers

The special registers are used for tracking program state, and handling the stack.

Special registers are referenced with `$$`. They are

`$$PC` the program couner, tracks the current position in the MimB file being executed.

`$$SP` the stack pointer, tracks the next insertion position of the virtual stack. Often simply
points to the end of the current stack frame.

`$$BP` the base pointer, tracks the location of the start of the current stack frame.


## General Registers

There are 64 general purpose processing registers. They are referenced with `$`.

i.e. `$0`, `$1`, ..., `$63`.

## Stack

The stack is used for reserving space for program execution.

## Internal Constats

Inernal constants are used for tracking virtual machine configuration, such as the stack size.

They are read with `$$$_` and can be thought of as simply replacing `$$$_` with the value in the
configuration. As such, they are used with immediate instructions, not register instructions.

## Program

The actual sequence of MimB code being executed by MimVM.

# MimASM

MimASM is an assembler language, buily by MimC, that is designed to roughly 1-to-1 mirror MimB code.

Code is broken into 3 main parts, configuration, static definitions, program code.

The compiler automatically jumps to the start of program code when executing.

Here is an example "Hello World" program.

```mimasm
; MimASM 0.1.0

; Configuration
config:
    _stack 2000000; 2MB Stack Space
    _iomode 1; Use VM IO ($$$_STDOUT, $$$_STDIN, $$$_STDERR)
    _bangmethods 1; Enable special bang-methods
    jump :main; Begin execution

; Data
data:
    hello: "Hello, World!\n";
    hello_len: :hello_len - :hello;

main:
    load $0, :hello;
    load $1, :hello_len;
    load $2, $$$_STDOUT;
    !write;
    load $0, $2;
    !flush;
    load $0, 0;
    !exit;
```

```mim
using STD::IO;
define Program {
    default mapping Main() {
        IO::PrintLn("Hello, World!");
    }
}
```

## Labels

Labels are names followed by a colon `foo:`, they mark a location in the source code in the 
equivalent machine code, making it easier to refer to and more resiliant to code cange.

In this respect, labels refer to program addresses. These can be referred to like immediates with
a prefix colon `:foo`. In the hello world code above, `hello:` occurs after a single other
instruction (`jump :main`), as such, it is at the 4th byte of program code, so `:hello` world be
equivalent to the value `4`.

## Configurations

Configurations generate zero bytes of the MimB executable body, instead, the are included in the
header section of te MimB file. The are called with a prefix underscore `_foo`.