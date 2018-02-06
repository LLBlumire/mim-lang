# MimVM, MimASM, MimB

MimVM is a virtual machine that executes MimB code. MimB is a binary formatted specification of Mim code.

MimB can be generated from either Mim, or MimASM, with MimC.

MimASM provides a roughly one-to-one conversion into MimB, Mim is more high level.

MimB is broken into sections. The header, which provides configuration options for the MimVM. The Typedef section, which defines the types that will be used and reffered to. Types in MimVM are structurally weak, so duplicate definitions are unnecesary.

## Configuration

## Typedef

MimVM is type system aware. It knows the structural type of data. This is important, as all data is kept behind either strong or weak pointers or buffer pointers, and thus the underlying data representation at the pointer being statically known is significant.

These are declared in the typedef section.

For example, the following declared types

```mim
// Mim 0.1.0

product Human {
    date_of_birth: u64,
    name: String,
    pet: Option<Pet>,
    gender: Gender
}
sum Pet {
    product Dog {
        breed: String
    }
    product Cat {
        color: String
    }
    product Unknown {
        type: String
    }
}
sum Gender {
    product Male { },
    product Female { },
    product Other { }
}
```

Would compile structurally, both Dog, Cat, and Unknown variants of Pet have the exact same underlying structural form. All variants of Gender are empty products, which take up a single byte (same as `Unit`). The following MimASM would generate equivalent MimB.

```mimasm
; MimASM 0.2.0

section typedef:
    utf32:
        bytes 4;
    String:
        buffer :utf32;
    Pet_CC_Dog:
    Pet_CC_Cat:
    Pet_CC_Unknown:
        begin_product;
            compose 0 :String;
        end_product;
    Option_TT_Pet_ZZ_Some:
    Pet:
        begin_sum;
            variant 0 :Pet_CC_Dog;
            variant 1 :Pet_CC_Cat;
            variant 2 :Pet_CC_Unknown;
        end_sum;
    Gender_CC_Male:
    Gender_CC_Female:
    Gender_CC_Other:
    Option_TT_Pet_ZZ_None:
        bytes 1;
    Gender:
        begin_sum;
            variant 0 :Gender_CC_Male;
            variant 1 :Gender_CC_Female;
            variant 2 :Gender_CC_Other;
        end_sum;
    Option_TT_Pet:
        begin_sum;
            variant 0 :Option_TT_Pet_CC_Some;
            variant 1 :Option_TT_Pet_CC_None;
        end_sum;
    uint64:
        bytes 8;
    Human:
        begin_product;
            compose 0 :uint64;
            compose 1 :String;
            compose 2 :Option_TT_Pet;
            compose 3 :Gender;
        begin_product;
```

All data is moved and interacted with by reference to it's type. Moving things to and from registers requires a type, to and from the virtual stack requires a type. To and from the virtual heap requires a type.

This allows allocations to be made efficiently, and the format of everything as being a reference counter strong pointer, reference counted weak pointer, or reference counted wide pointer, become irrelevant.

The underlying structure of data is used to make data live and avaliable, and reason about the structure.

For example, the following function paired with the above.

```mim
// Mim 0.1.0

mapping Example() {
    let x = Human {
        date_of_birth: 1997,
        name: "Lucy",
        pet: None,
        gender: Gender::Female
    };
}
```

Might generate something equivalent to the following MimASM (alongside the previous definitions).

```mimasm
; MimASM 0.2.0

section typedef:
    Unit:
        bytes 1;
    TM_Example:
        begin_frame;
            nodomain;
            local 0 :Human;
            local 1 :uint64;
            local 2 :String;
            local 3 :Option_TT_Pet;
            local 4 :Option_TT_Pet_CC_None;
            local 5 :Gender;
            local 6 :Gender_CC_Female;
            range :Unit;
            body :MM_Example;
        end_frame;

section data:
    MM_Example_L2_String:
        data_utf32 "Lucy";
    MM_Example_L2_String_Len:
        data 16;

section main:
    MM_Example:
        begin_frame_body :TM_Example;
            local_bytes_init 1 0d1997;
            local_buffer_init 2 :MM_Example_L2_String :MM_Example_L2_String_Len;
            local_bytes_init 4 0x0;
            local_sum_set 3 1 4;
            local_bytes_init 6 0x0;
            local_sum_set 5 1 6;
            local_product_set 0 0 1;
            local_product_set 0 1 2;
            local_product_set 0 2 3;
            local_product_set 0 3 5;
        end_frame_body;
```