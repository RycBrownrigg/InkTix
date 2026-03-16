/// Core tests are included inline in lib.rs under #[cfg(test)]
/// This module provides additional test helpers if needed.

pub struct CoreTestHelpers;

impl CoreTestHelpers {
    /// Helper to verify contract initialization
    pub fn verify_empty_contract_state(totals: (u32, u32, u32, u64, u32, u32)) {
        assert_eq!(totals, (0, 0, 0, 0, 0, 0));
    }
}
