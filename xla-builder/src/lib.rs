mod builder;
pub use builder::Builder;

#[cfg(test)]
mod test {
    use std::thread::Builder;

    #[test]
    fn create_builder() {
        let b = Builder::new("test");
    }
}
