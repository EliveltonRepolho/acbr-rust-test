use acbr_rust::{get_version};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_version() {
        let version = get_version();
        assert!(!version.is_none());
        assert_eq!(version, Some(String::from("1.0.0.104")));
    }
    
}