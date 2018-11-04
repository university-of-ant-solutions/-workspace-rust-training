// An attribute to hide warnings for unused code.
#![allow(dead_code)]

// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
enum WebEvent {
    // An `enum` may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or like structures.
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) -> i64 {
    match event {
        WebEvent::PageLoad => 1,
        WebEvent::PageUnload => 2,
        // Destructure `c` from inside the `enum`.
        WebEvent::KeyPress(c) => 3,
        WebEvent::Paste(s) => 4,
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            x + y
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let pressed = WebEvent::KeyPress('x');
        // `to_owned()` creates an owned `String` from a string slice.
        let pasted  = WebEvent::Paste("my text".to_owned());
        let click   = WebEvent::Click { x: 20, y: 80 };
        let load    = WebEvent::PageLoad;
        let unload  = WebEvent::PageUnload;

        assert_eq!(inspect(pressed), 3);
        assert_eq!(inspect(pasted), 4);
        assert_eq!(inspect(click), 100);
        assert_eq!(inspect(load), 1);
        assert_eq!(inspect(unload), 2);
    }
}
