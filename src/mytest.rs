#[cfg(test)]

mod tests {
    use std::fs;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn it_file() {
        let contents = fs::read_to_string("data.dat")
            .expect("Something went wrong reading the file");
        assert_eq!(contents, "hello world");

    }
}