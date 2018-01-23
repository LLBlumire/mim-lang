# Mappings

## Summary

Mappings are a type of item that provide methods of data transformation.

## Syntax

The `mapping` keyword defines a new mapping. It expects an identity, an optional list of type parameters, a list of value parameters, and an optional return type. Given the absense of a return type, a type of `()` is assumed (the empty tuple).

The `lambda` keyword defines an inline mapping. It creates a new mapping that closes over it's environment. It expects an optional list of type parameters, then a list of value paramaters a return type, then a body.

Both mappings and lambda's can have their type paramaters constrained with the `constrain` keyword, which can limit type parameters to certain protocols.

The style mandates that they are named in `UpperCamelCase`

```mim
using Std::Mapping;
using Std::Ops::Add;
define MappingsExample {
    mapping AddTwo(x: u32): u32 {
        x + 2
    }
    mapping AddTogether(x: u32, y: u32): u32 {
        x + y
    }
    mapping AddCurried(x: u32): Mapping<(u32), u32> {
        lambda (y: u32): u32 {
            x + y
        }
    }
    mapping AddTogetherWithCurry(x: u32, y: u32): u32 {
        (AddCurried(x))(y)
    }
    mapping GenericAdd<T, U, V>(x: T, y: V): U constrain
        T: Add<U, Result=T> 
    {
        x + y
    }
}
```