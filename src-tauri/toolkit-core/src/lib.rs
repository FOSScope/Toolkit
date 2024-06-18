pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from toolkit-core!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = greet("world");
        assert_eq!(result, "Hello, world! You've been greeted from toolkit-core!");
    }
}
