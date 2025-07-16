use creusot_contracts::*;

#[requires(a@ < i64::MAX@)]
#[ensures(result@ == a@ + 1)]
pub fn add_one(a: i64) -> i64 {
    a + 1
}

#[requires(n@ * (n@ + 1) / 2 <= u64::MAX@)]
#[ensures(result@ == n@ * (n@ + 1) / 2)]
pub fn sum_up_to(n: u64) -> u64 {
    let mut sum = 0;
    let mut i = 0;

    #[invariant(i@ <= n@)]
    while i < n {
        i += 1;
        sum += i;
    }
    sum
}

#[test]
pub fn test_sum_up_to() {
    // 1 + 2 + 3 + 4 = 10
    assert_eq!(sum_up_to(4), 10);
}
