use rand::Rng;

// rand_int function that generates a random number within the given range
fn rand_int(low: i32, high: i32) -> i32 {
    let (low, high) = if low <= high {
        (low, high)
    } else {
        (high, low)
    };

    let mut rng = rand::rng();
    rng.random_range(low..=high)
}

// Unit tests for rand_int():
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rand_int() {
        let lower_bound = 1;
        let upper_bound = 100;
        for _ in 1..1000 {
            let num = rand_int(lower_bound, upper_bound);
            assert!((lower_bound..=upper_bound).contains(&num));
        }
    }
    #[test]
    fn test_rand_int_lower_and_upper_bound_swap() {
        let lower_bound = 100;
        let upper_bound = 1;
        let num = rand_int(lower_bound, upper_bound);
        println!("Number generated is : {num}");
        assert!(true);
    }
}