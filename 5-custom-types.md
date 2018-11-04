---
layout: post
title: custom types
tags: [rust, custom, type]
---

Let’s create a new library project called custom-type:

```
$ cargo new custom-types --lib
     Created library `custom-types` project
$ cd custom-types
```

Then run `cargo test` command to runs all tests in our project

### Structures

There are three types of structures ("structs") that can be created using the struct keyword:

- Tuple structs, which are, basically, named tuples.
- The classic C structs
- Unit structs, which are field-less, are useful for generics.

[Example](/custom-types/src/structures.rs)

### Enums

The enum keyword allows the creation of a type which may be one of a few different variants. Any variant which is valid as a struct is also valid as an enum.

#### use

The use declaration can be used so manual scoping isn't needed:

```
// An attribute to hide warnings for unused code.
#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // Explicitly `use` each name so they are available without
    // manual scoping.
    use Status::{Poor, Rich};
    // Automatically `use` each name inside `Work`.
    use Work::*;
}
```

#### C-like

enum can also be used as C-like enums.

```
// An attribute to hide warnings for unused code.
#![allow(dead_code)]

// enum with implicit discriminator (starts at 0)
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    // `enums` can be cast as integers.
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}
```

[Example](/custom-types/src/enums.rs)

#### Testcase: linked-list

A common use for enums is to create a linked-list.

[Example](/custom-types/src/linked_list.rs)

#### constants

[https://doc.rust-lang.org/rust-by-example/custom_types/constants.html](https://doc.rust-lang.org/rust-by-example/custom_types/constants.html)
