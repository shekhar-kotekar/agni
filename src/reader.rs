struct Reader {}

impl Reader {
    fn csv(csv_path: &str) -> Result<Reader, Box<dyn std::error::Error>> {
        Ok(Reader {})
    }
}
