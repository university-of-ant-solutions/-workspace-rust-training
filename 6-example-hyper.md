---
layout: post
title: example hyper
tags: [rust, example, hyper]
---

Let’s create a new library project called example-hyper:

```
$ cargo new example-hyper --lib
     Created library `example-hyper` project
$ cd example-hyper
```

Let’s start by making a “Hello, World!” server, and expand from there.

First, we need our dependencies. Let’s tell Cargo about our dependencies by having this in the Cargo.toml.

```
[dependencies]
hyper = "0.12"
```
