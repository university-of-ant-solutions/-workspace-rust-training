---
layout: post
title: unit test with rust
tags: [rust, unit test]
---

### The Anatomy of a Test Function

Let’s create a new library project called adder:

```
$ cargo new adder --lib
     Created library `adder` project
$ cd adder
```

Filename: src/lib.rs

```
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

Note the #[test] annotation before the fn line: this attribute indicates this is a test function, so the test runner knows to treat this function as a test. We could also have non-test functions in the tests module to help set up common scenarios or perform common operations, so we need to indicate which functions are tests by using the #[test] attribute.

The function body uses the assert_eq! macro to assert that 2 + 2 equals 4. This assertion serves as an example of the format for a typical test. Let’s run it to see that this test passes.

The cargo test command runs all tests in our project

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

- https://doc.rust-lang.org/rust-by-example/cargo/test.html

- https://doc.rust-lang.org/book/second-edition/ch11-00-testing.html
