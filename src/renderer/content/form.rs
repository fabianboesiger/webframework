pub trait Form {
    fn form(&self) -> String;
    fn populate(input: String);
}