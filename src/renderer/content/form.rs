pub trait Form {
    fn html(&self) -> Vec<u8>;
}