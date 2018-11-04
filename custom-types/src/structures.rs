#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

// A tuple struct
struct Pair(i32, f32);

// A unit struct
struct Nil;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        // Instantiate a `Point`
        let point: Point = Point { x: 0.3, y: 0.4 };
        assert_eq!(point.x, 0.3);
        assert_eq!(point.y, 0.4);

        // Make a new point by using struct update syntax to use the fields of our other one
        let new_point = Point { x: 0.1, ..point };
        assert_eq!(new_point.x, 0.1);
        assert_eq!(new_point.y, 0.4);

        // Destructure the point using a `let` binding
        let Point { x: my_x, y: my_y } = point;
        assert_eq!(my_x, 0.3);
        assert_eq!(my_y, 0.4);

        // Instantiate a tuple struct
        let pair = Pair(1, 0.1);
        assert_eq!(pair.0, 1);
        assert_eq!(pair.1, 0.1);

        // Destructure a tuple struct
        let Pair(integer, decimal) = pair;
        assert_eq!(integer, 1);
        assert_eq!(decimal, 0.1);

        // Create struct with field init shorthand
        let name = "Peter";
        let age = 27;
        let peter = Person { name, age };
        assert_eq!(peter.name, "Peter");
        assert_eq!(peter.age, 27);

        let _rectangle = Rectangle {
            // struct instantiation is an expression too
            p1: Point { x: my_y, y: my_x },
            p2: point,
        };
        // Instantiate a unit struct
        let _nil = Nil;
    }
}
