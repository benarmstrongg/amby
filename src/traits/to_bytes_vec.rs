pub trait ToBytesVec {
    fn to_bytes(self) -> Vec<u8>
    where
        Self: Into<Vec<u8>>,
    {
        self.into()
    }
}

// impl<T: Into<Vec<u8>> + ToBytes, I: Iterator<Item = T>> ToBytes for I {
//     fn to_bytes(self) -> Vec<u8> {}
// }
