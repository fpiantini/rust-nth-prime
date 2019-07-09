
pub mod prime_numbers {

    use std::iter;

    // *******************************************************
    //    OLD CONVENTIONAL STYLE
    pub fn is_prime_oldf(val: u32) -> bool {

        match val {
            1 => false,
            val if val != 2 && val % 2 == 0 => false,
            val if val > 8 => {
                let upper_limit = (val as f64).sqrt() as u32;
                let mut cnt = 3;
                while cnt <= upper_limit {
                    if val % cnt == 0 {
                        return false;
                    }
                    cnt += 2;
                }
                true
            },
            _ => true
        }
    }

    pub fn nth_prime_oldf(n: u32) -> u32 {

        let mut num  = 2;

        // Iterator that returns prime numbers
        let mut prime_iterator = iter::from_fn(move || {
            while !is_prime_oldf(num) {
                num += 1;
            }
            num += 1;
            Some(num - 1)
        });

        prime_iterator.nth(n as usize).unwrap()

    }
    // *******************************************************

    // *******************************************************
    //    WITH ITERATOR VERSION 1
    fn is_prime_with_iter1(n: u32) -> bool {

        match n {
            1 => false,
            n if n != 2 && n % 2 == 0 => false,
            n if n > 8 => {
                let upper_limit = (n as f64).sqrt() as u32;
                let mut odd  = 3;

                // Iterator that returns odd numbers
                let odd_numbers = iter::from_fn(move || {
                    odd += 2;
                    Some(odd - 2)
                });

                for e in odd_numbers {
                    if n % e == 0 {
                        return false;
                    }
                    if e >= upper_limit {
                        return true;
                    }
                }
                true
            },
            _ => true
        }

    }

    pub fn nth_prime_iter1(n: u32) ->u32 {

        let mut num  = 2;

        // Iterator that returns prime numbers
        let mut prime_iterator = iter::from_fn(move || {
            while !is_prime_with_iter1(num) {
                num += 1;
            }
            num += 1;
            Some(num - 1)
        });

        prime_iterator.nth(n as usize).unwrap()

    }
    // *******************************************************

    // *******************************************************
    //     ELEGANT BUT INEFFICIENT
    pub fn is_prime_elegant_but_inefficient(n: u32) -> bool {
        let upper_limit = (n as f64).sqrt() as u32 + 1;
        ! (2..upper_limit).any(|i| n % i == 0)
    }

    pub fn nth_prime_elegant_but_inefficient(n: u32) -> u32 {
        (2..).filter(|c| is_prime_elegant_but_inefficient(*c)).nth(n as usize).unwrap()
    }
    // *******************************************************

    // *******************************************************
    //     try_for_each
    pub fn is_prime_try_for_each(n: u32) -> bool {
        match n {
            1 => false,
            n if n != 2 && n % 2 == 0 => false,
            n if n > 8 => {
                let upper_limit = (n as f64).sqrt() as u32 + 1;
                match (2..upper_limit).try_for_each(|i| {
                    match n % i {
                        0 => None,
                        _ => Some(())
                    }
                }) {
                    None => false,
                    Some(()) => true
                }
            },
            _ => true
        }
    }

    pub fn nth_prime_try_for_each(n: u32) -> u32 {
        // (2..).filter(|c| is_prime_try_for_each(*c)).nth(n as usize).unwrap()
        let mut num  = 2;

        // Iterator that returns prime numbers
        let mut prime_iterator = iter::from_fn(move || {
            while !is_prime_try_for_each(num) {
                num += 1;
            }
            num += 1;
            Some(num - 1)
        });

        prime_iterator.nth(n as usize).unwrap()

    }
    // *******************************************************

}

#[cfg(test)]
mod tests {

    use super::prime_numbers as primes;

    #[test]
    fn test_two_is_prime_oldf() {
        assert!(primes::is_prime_oldf(2));
    }

    #[test]
    fn test_three_is_prime_oldf() {
        assert!(primes::is_prime_oldf(3));
    }

    #[test]
    fn test_13_is_prime_oldf() {
        assert!(primes::is_prime_oldf(13));
    }

    #[test]
    fn test_big_prime_oldf() {
        assert!(primes::is_prime_oldf(104743));
    }

    #[test]
    fn test_very_big_prime_oldf() {
        assert!(primes::is_prime_oldf(15485867));
    }

    #[test]
    fn test_very_big_not_prime_oldf() {
        assert!(!primes::is_prime_oldf(15485869));
    }

    #[test]
    fn test_nth_prime_first_prime_oldf() {
        assert_eq!(primes::nth_prime_oldf(0), 2);
    }

    #[test]
    fn test_nth_prime_second_prime_oldf() {
        assert_eq!(primes::nth_prime_oldf(1), 3);
    }

    #[test]
    fn test_nth_prime_sixth_prime_oldf() {
        assert_eq!(primes::nth_prime_oldf(5), 13);
    }

    #[test]
    fn test_nth_prime_big_prime_oldf() {
        assert_eq!(primes::nth_prime_oldf(10000), 104743);
    }

    #[test]
    fn test_nth_prime_very_big_prime_oldf() {
        assert_eq!(primes::nth_prime_oldf(1000000), 15485867);
    }

    #[test]
    fn test_nth_prime_big_prime_iter1() {
        assert_eq!(primes::nth_prime_iter1(10000), 104743);
    }

    #[test]
    fn test_nth_prime_very_big_prime_iter1() {
        assert_eq!(primes::nth_prime_iter1(1000000), 15485867);
    }

    #[test]
    fn test_nth_prime_big_prime_ebi() {
        assert_eq!(primes::nth_prime_elegant_but_inefficient(10000), 104743);
    }

    #[test]
    fn test_nth_prime_very_big_prime_ebi() {
        assert_eq!(primes::nth_prime_elegant_but_inefficient(1000000), 15485867);
    }

    #[test]
    fn test_nth_prime_big_prime_try_for_each() {
        assert_eq!(primes::nth_prime_try_for_each(10000), 104743);
    }

    #[test]
    fn test_nth_prime_very_big_prime_try_for_each() {
        assert_eq!(primes::nth_prime_try_for_each(1000000), 15485867);
    }

}

