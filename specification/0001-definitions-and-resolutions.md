# Definitions and Resolutions

## Summary

Definitions are the basic component of Mim programs. All things in Mim must exist inside definitions. This includes mappings, product types, sum types, type aliases, protocols, protocol implementation, and all other kinds of items.

## Syntax

There are a number of keywords related to Definitions and their resolutions. These are `define`, `defines`, `default`, `public`, `using`.

`define` is used to create new definitions. It expects a following name, and body. The name is recommended by style to be in `CapitalCamelCase`.

```mim
define Foo {}
```

Although definitions can exist in any file, it is recommended that a definition should be in a file of the same name. Thus, the above would be in a file called `Foo.mim`.

`using` allows you to use the contents of another definition.

Items placed in a definition are by default private, and not exposed by the definition. `public` allows these to be exposed externally.

```mim
define Foo {
    public constant VALUE: u32 = 1337;
}
```
```mim
using Foo;
define Bar {
    constant BAR_VALUE: u32 = Foo::VALUE;
}
```

`defines` is used to associate definitions from seperate files. It expects only the name of a definition.

```mim
define Foo {
    public defines Bar;
}
```
```mim
define Bar {
    public constant VALUE: u32 = 1337;
}
```
```mim
using Foo;
define Baz {
    constant BAZ_VALUE: u32 = Foo::Bar::VALUE;
}
```

`default` allows you to treat the usage of a definition as the usage of an item within that definition. `default` items are `public` implicitly. They cannot be made private.

```mim
define Foo {
    default constant VALUE: u32 = 1337;
}
```
```mim
using Foo;
define Bar {
    constant BAR_VALUE: u32 = Foo;
}
```