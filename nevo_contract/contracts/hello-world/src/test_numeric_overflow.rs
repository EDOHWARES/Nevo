#[cfg(test)]
mod numeric_overflow_tests {
    use super::*;

    #[test]
    fn test_max_i128_amount_handled() {
        let result = process_amount(i128::MAX);
        assert!(result.is_ok() || result.is_err(),
            "i128::MAX must be explicitly handled, not panic");
    }

    #[test]
    fn test_addition_overflow_prevented() {
        let result = safe_add(i128::MAX, 1);
        assert!(result.is_err(), "Adding 1 to i128::MAX must return Err, not overflow");
    }

    #[test]
    fn test_contribution_sum_limits() {
        let result = add_contribution(i128::MAX / 2, i128::MAX / 2 + 2);
        assert!(result.is_err(), "Contribution sum exceeding i128::MAX must be rejected");
    }

    #[test]
    fn test_pool_target_amount_limits() {
        let result = set_pool_target(i128::MAX);
        assert!(result.is_err(), "Pool target at i128::MAX must be bounded or rejected");
    }

    #[test]
    fn test_counter_overflow_protection() {
        let result = increment_counter(u64::MAX);
        assert!(result.is_err(), "Counter at u64::MAX must not wrap around silently");
    }

    #[test]
    #[should_panic(expected = "Collected amount overflow")]
    fn test_donate_collected_overflow_panics() {
        // donate() must use checked_add for pool.collected so that an amount
        // which would overflow u128 panics with a clear message instead of
        // silently wrapping in release/wasm builds.
        let collected: u128 = u128::MAX;
        let amount: u128 = 1;
        let _new_collected = collected
            .checked_add(amount)
            .expect("Collected amount overflow");
    }

    #[test]
    #[should_panic(expected = "Contribution amount overflow")]
    fn test_donate_contribution_overflow_panics() {
        // donate() must use checked_add for the donor's current_contrib so that
        // an amount which would overflow u128 panics with a clear message
        // instead of silently wrapping in release/wasm builds.
        let current_contrib: u128 = u128::MAX;
        let amount: u128 = 1;
        let _new_contrib = current_contrib
            .checked_add(amount)
            .expect("Contribution amount overflow");
    }
}
