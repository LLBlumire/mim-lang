# MimVM

MimVM is a virtual machine that evaluates the results of compiling Mim. It is the primary target of MimC.

## Model

Memory in MimVM is represented as reference counted pointers. These each with them have associated the number of strong pointers to the memory location, as well as whether the pointer is strong or weak. If the number of pointers falls to zero, the memory is freed. 

MimVM instructions are all exactly 

```

```

## Registers

Registers in MimVM are virtual, they do not necessarily represent actual in hardware registers.




## Opcodes

<table>
    <tr>
        <th>Code</th>
        <th>Mmemonic</th>
        <th coldpsan="8">Encoding</th>
    </tr>
    <tr>
        <td>0</td>
        <td>NOOP</td>
        <td>00</td>
        <td>??</td>
        <td>??</td>
        <td>??</td>
        <td>??</td>
        <td>??</td>
        <td>??</td>
    </tr>
</table>