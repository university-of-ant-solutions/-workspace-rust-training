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
