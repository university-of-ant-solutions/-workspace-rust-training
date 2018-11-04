---
layout: post
title: example hyper
tags: [rust, example, hyper]
---

Let’s create a new library project called example-hyper:

```
$ cargo new example-hyper
     Created binary (application) `example-hyper` project
$ cd example-hyper
```

Let’s start by making a “Hello, World!” server, and expand from there.

First, we need our dependencies. Let’s tell Cargo about our dependencies by having this in the Cargo.toml.

```
[dependencies]
hyper = "0.12"
```

Now lets start on our main.rs, and add some imports:

```
extern crate hyper;
```

We also need to use a few things:

```
use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;
```

### Creating a Service

A Service lets you define how to respond to incoming requests. While it is possible to implement the trait directly, there are a few patterns that are common when using Hyper. We’ve included some helpers for when these patterns fit our needs.

In this example, we don’t have any state to carry around, so we really just need a simple function:

```
const PHRASE: &str = "Hello, World!";

fn hello_world(_req: Request<Body>) -> Response<Body> {
    Response::new(Body::from(PHRASE))
}
```

### Starting the Server
Lastly, we need to hook up our hello_world service into a running hyper Server.

We’ll dive in to the specifics of some of these things in another guide.

```
// This is our socket address...
let addr = ([127, 0, 0, 1], 3000).into();

// A `Service` is needed for every connection, so this
// creates one from our `hello_world` function.
let new_svc = || {
    // service_fn_ok converts our function into a `Service`
    service_fn_ok(hello_world)
};

let server = Server::bind(&addr)
    .serve(new_svc)
    .map_err(|e| eprintln!("server error: {}", e));

// Run this server for... forever!
hyper::rt::run(server);
```

To compile it, type:

```
$ cd hello
$ cargo build
```

And then run it:

```
$ ./target/debug/example-hyper
```

We can also use cargo run to compile and then run it, all in one step:

```
$ cargo run
```

