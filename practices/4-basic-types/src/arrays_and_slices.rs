#[cfg(test)]
mod test {
    use std::mem;
    #[test]
    fn test_arrays_and_slices() {
       // Fixed-size array (type signature is superfluous)
        let xs: [i32; 5] = [1, 2, 3, 4, 5];

        assert_eq!(xs[0], 1);
        assert_eq!(xs[1], 2);
        assert_eq!(xs.len(), 5);
        // Arrays are stack allocated
        assert_eq!(mem::size_of_val(&xs), 20); // 20 bytes
    }
}
