pub trait ExtractString {
    fn from_d_quotes(self) -> Self;
}

impl ExtractString for String {
    fn from_d_quotes(self) -> Self {
        self.replace('"', "")
    }
}
