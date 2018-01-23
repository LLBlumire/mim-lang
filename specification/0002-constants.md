# Constants

## Summary

Constants are a type of item that allow for immutable data definitions. The value assigned to a constant is substituted in all other locations. These values must be compile time computable.

## Syntax

Constants are created with the keyword `constant` which expects a name, type, and value. Names should be in `UPPER_SNAKE_CASE` per the standard style.

```mim
using Std::Collections::Vector;
define ConstantsExample {
    constant FOO: f64 = 10.3;
    constant BAR: String  = "Hello, World";
    constant TEN: u32 = Ten();
    constant VEC: Vector<u32> = Vector::<u32>::New();

    mapping Ten(): u32 {
        10
    }
}
```