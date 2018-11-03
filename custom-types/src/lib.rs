// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

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
    }
}
