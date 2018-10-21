---
layout: post
title: basic type in rust
tags: [rust, basic, type]
---

Let’s create a new library project called basic-type:

```
$ cargo new basic-types --lib
     Created library `basic-types` project
$ cd basic-types
```

### Primitives

Rust provides access to a wide variety of primitives. A sample includes:

#### Scalar types

| Size (bits)  | Unsigned integer                     | Signed integer           | Floating-point |
|--------------|--------------------------------------|--------------------------|----------------|
| 8            | u8 (0 to 2^8 – 1)                    | i8 (−2^7 to 2^7 − 1)     |                |
| 16           | u16 (0 to 2^16 − 1)                  | i16 (−2^15 to 2^15 − 1)  |                |
| 32           | u32 (0 to 2^32 − 1)                  | i32 (−2^31 to 2^31 − 1 ) | f32            |
| 64           | u64 (0 to 2^64 − 1)                  | i64 (−2^63 to 2^63 − 1)  | f64            |
| Machine word | usize 0 to either 2^32 −1 or 2^64 −1 | isize                    |                |

#### Compound Types

##### Arrays and Slices

An array is a collection of objects of the same type T, stored in contiguous memory. Arrays are created using brackets [], and their size, which is known at compile time, is part of their type signature [T; size].

Slices are similar to arrays, but their size is not known at compile time. Instead, a slice is a two-word object, the first word is a pointer to the data, and the second word is the length of the slice. The word size is the same as usize, determined by the processor architecture eg 64 bits on an x86-64. Slices can be used to borrow a section of an array, and have the type signature &[T].

[Example](/basic-types/src/tuples.rs)

##### Tuples

A tuple is a collection of values of different types. Tuples are constructed using parentheses (), and each tuple itself is a value with type signature (T1, T2, ...), where T1, T2 are the types of its members. Functions can use tuples to return multiple values, as tuples can hold any number of values.

[Example](/basic-types/src/tuples.rs)

### Mutability

Variable bindings are immutable by default, but this can be overridden using the mut modifier.

```
fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Error!
    _immutable_binding += 1;
    // FIXME ^ Comment out this line
}
```

The compiler will throw a detailed diagnostic about mutability errors.

### Links

- https://doc.rust-lang.org/rust-by-example/primitives.html#scalar-types

- https://doc.rust-lang.org/rust-by-example/variable_bindings/mut.html

