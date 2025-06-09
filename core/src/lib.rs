pub mod events;
pub mod state;

/// Simple addition function used for unit tests.
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add(2, 2), 4);
    }
}

/// Placeholder to avoid empty crate warnings.
pub fn placeholder() {}
