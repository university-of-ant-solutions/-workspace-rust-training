---
layout: post
title: modules in rust
tags: [rust, modules]
---

Let’s create a new library project called modules:

```
$ cargo new modules --lib
     Created library `modules` project
$ cd modules
```

Then run `cargo test` command to runs all tests in our project

```
cargo test
   Compiling adder v0.1.0 (file:///<PATH_TO_WORKSPACE>/rust-training/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 1.87s
     Running target/debug/deps/adder-c2f065bd489d9345

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Links

- https://doc.rust-lang.org/rust-by-example/mod.html
