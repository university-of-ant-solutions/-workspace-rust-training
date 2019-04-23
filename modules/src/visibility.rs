// Use the `pub` modifier to override default visibility.
pub fn function() -> i32 {
    32
}

// Items in modules default to private visibility.
fn private_function() -> i32 {
    16
}

// Items can access other items in the same module,
// even when private.
pub fn indirect_access() -> i32 {
    private_function()
}
