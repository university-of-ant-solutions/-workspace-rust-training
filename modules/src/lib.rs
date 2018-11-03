mod visibility;

#[cfg(test)]
mod test {
	use super::*;
    #[test]
    fn test_visibility() {
        assert_eq!(visibility::function(), 32);
        assert_eq!(visibility::indirect_access(), 16);
    }
}