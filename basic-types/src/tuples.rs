// Tuples can be used as function arguments and as return values
pub fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables
    let (integer, boolean) = pair;

    (boolean, integer)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_tuples() {
        // A tuple with a bunch of different types
        let long_tuple = (1u8, 2u16, 3u32, 4u64,
                          -1i8, -2i16, -3i32, -4i64,
                          0.1f32, 0.2f64,
                          'a', true);
        // Values can be extracted from the tuple using tuple indexing
        assert_eq!(long_tuple.0, 1);
        assert_eq!(long_tuple.1, 2);

        // Tuples can be tuple members
        let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
        // Tuples are printable
        assert_eq!(tuple_of_tuples.0, (1u8, 2u16, 2u32));

        // But long Tuples cannot be printed
        // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
        // println!("too long tuple: {:?}", too_long_tuple);
        // TODO ^ Uncomment the above 2 lines to see the compiler error

        let pair = (1, true);
        let reversed = reverse(pair);
        assert_eq!(reversed.0, true);
        assert_eq!(reversed.1, 1);

        // To create one element tuples, the comma is required to tell them apart
        // from a literal surrounded by parentheses
        // println!("one element tuple: {:?}", (5u32,));
        // println!("just an integer: {:?}", (5u32));

        // tuples can be destructured to create bindings
        // let tuple = (1, "hello", 4.5, true);

        // let (a, b, c, d) = tuple;
        // println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

        // let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
        // println!("{:?}", matrix);
    }
}
