pub trait ToSlice {
    fn to_slice<const T: usize>(self) -> [u8; T];
}

impl ToSlice for String {
    fn to_slice<const T: usize>(self) -> [u8; T] {
        let mut buf: [u8; T] = [0; T];
        let len = self.len();
        buf[..len].copy_from_slice(&self.as_bytes());
        buf
    }
}
