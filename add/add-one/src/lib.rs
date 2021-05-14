use rand::thread_rng;

pub fn add_one(x: i32) -> i32 {
    x + 1
}


// cargo test -p add_one

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_one_test() {
        assert_eq!(2, add_one(1));
    }
}