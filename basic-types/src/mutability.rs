#[cfg(test)]
mod test {
    #[test]
    fn mutability() {
        let _immutable_binding = 1;
        let mut mutable_binding = 1;

        assert_eq!(mutable_binding, 1);

        // Ok
        mutable_binding += 1;

        assert_eq!(mutable_binding, 2);

        // Error!
        // _immutable_binding += 1;
        // FIXME ^ Comment out this line
    }
}

