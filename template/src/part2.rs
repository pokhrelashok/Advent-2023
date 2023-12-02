use std::collections::HashMap;

pub fn process(input: &str) -> u32 {
    let lines = input.split("\n").map(|v| v.trim());
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = process("");
        assert_eq!(result, 0);
    }
}
