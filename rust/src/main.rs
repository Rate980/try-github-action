fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_pass() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_fail() {
        assert_eq!(2 + 2, 5);
    }
}
