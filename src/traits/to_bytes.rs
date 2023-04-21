pub trait ToBytes {
    fn to_bytes(self) -> Vec<u8>
    where
        Self: Into<Vec<u8>>,
    {
        self.into()
    }
}
