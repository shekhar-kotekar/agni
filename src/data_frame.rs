struct DataFrame {}

impl DataFrame {
    fn first(&self) -> Result<&str, Box<dyn std::error::Error>> {
        Ok("first")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let df = DataFrame {};
        assert_eq!(df.first().unwrap(), "first");
    }
}
