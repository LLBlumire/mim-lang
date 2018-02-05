```mimasm
; MimASM 0.1.0
; Hello World From Mim

; Configuration
config:
    _stack 2000000;
    _iomode 1;
    _bangmethods 1;
    jump :main;

; Data
data:
    hello: "Hello, World!";
    hello_len: (:hello_len - :hello);

; Main
main:
    push $$BP;
    push $$SP;
    load $$BP, $$SP;
    load $$SP, ($$SP + 12);

    load $0, 12;
    load $1, ($$BP + 8);
    !strong; pushes 4 byte virtual strong pointer to the stack ($$BP + 8)
    load $0, @:hello_len;
    !reserve;
    load $0, :hello;
    load $1, @:hello_len;
    load $2, @(@($$BP + 8) + 0);
    !memcopy;
    load $0, @:hello_len;
    load $1, 4;
    load $2, @(@($$BP + 8) + 4);
    !memcopy;
    load $0, @:hello_len;
    load $1, 4;
    load $2, @(@($$BP + 8) + 8);
    !memcopy;
    
    load $0, @(@($$BP + 8) + 0);
    load $1, @(@($$BP + 8) + 4);
    load $2, $$$_STDOUT;
    !write;
    load $0, $2;
    !flush;

    load $$SP, @($$BP + 4);
    load $$BP, @($$BP + 0); 
    load $0, 0;
    !exit; 
```

```mim
// Mim 0.1.0
using STD::IO;
define Program {
    default mapping Main() {
        IO::Print("Hello, World");
    }
}
```