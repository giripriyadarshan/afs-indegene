pub trait ExtractString {
    fn remove_d_quotes(self) -> Self;
}

impl ExtractString for String {
    fn remove_d_quotes(self) -> Self {
        self.replace('"', "")
    }
}
