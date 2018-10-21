#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        // Variables can be type annotated.
        let logical: bool = true;
        assert_eq!(logical, true);

        let a_float: f64 = 1.0;  // Regular annotation
        assert_eq!(a_float, 1.0);

        let an_integer   = 5i32; // Suffix annotation
        assert_eq!(an_integer, 5);

        // Or a default will be used.
        let default_float   = 3.0; // `f64`
        assert_eq!(default_float, 3.0);

        let default_integer = 7;   // `i32`
        assert_eq!(default_integer, 7);

        // A type can also be inferred from context 
        // warning: value assigned to `inferred_type` is never read
        let mut inferred_type = 12; // Type i64 is inferred from another line
        inferred_type = 4294967296i64;
        assert_eq!(inferred_type, 4294967296);

        // A mutable variable's value can be changed.
        // warning: value assigned to `mutable` is never read
        let mut mutable = 12; // Mutable `i32`
        mutable = 21;
        assert_eq!(mutable, 21);

        // Error! The type of a variable can't be changed.
        // mutable = true;
        
        // Variables can be overwritten with shadowing.
        // let mutable = true;
    }
}
