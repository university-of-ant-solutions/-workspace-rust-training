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

#### Scalar types

| Size (bits)  | Unsigned integer                     | Signed integer           | Floating-point |
|--------------|--------------------------------------|--------------------------|----------------|
| 8            | u8 (0 to 2^8 – 1)                    | i8 (−2^7 to 2^7 − 1)     |                |
| 16           | u16 (0 to 2^16 − 1)                  | i16 (−2^15 to 2^15 − 1)  |                |
| 32           | u32 (0 to 2^32 − 1)                  | i32 (−2^31 to 2^31 − 1 ) | f32            |
| 64           | u64 (0 to 2^64 − 1)                  | i64 (−2^63 to 2^63 − 1)  | f64            |
| Machine word | usize 0 to either 2^32 −1 or 2^64 −1 | isize                    |                |

### Links

- https://doc.rust-lang.org/rust-by-example/primitives.html#scalar-types

