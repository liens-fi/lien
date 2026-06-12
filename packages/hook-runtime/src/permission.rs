//! Permission gating primitives shared by hooks that filter borrowers.

use std::collections::HashSet;

/// Stateless reputation lookup. In production this is backed by an on-chain
/// account; in tests we use the [`MemoryReputation`] implementation.
pub trait ReputationProvider: Send + Sync {
    /// Score in basis points. 10_000 = perfect, 0 = never seen.
    fn score(&self, borrower: &[u8; 32]) -> u16;

    /// Total successful repayments observed.
    fn repayment_count(&self, borrower: &[u8; 32]) -> u32;
}

#[derive(Default)]
pub struct MemoryReputation {
    scores: std::sync::RwLock<std::collections::HashMap<[u8; 32], (u16, u32)>>,
}

impl MemoryReputation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&self, borrower: [u8; 32], score: u16, repayments: u32) {
        self.scores.write().unwrap().insert(borrower, (score, repayments));
    }
}

impl ReputationProvider for MemoryReputation {
    fn score(&self, borrower: &[u8; 32]) -> u16 {
        self.scores.read().unwrap().get(borrower).map(|(s, _)| *s).unwrap_or(0)
    }

    fn repayment_count(&self, borrower: &[u8; 32]) -> u32 {
        self.scores.read().unwrap().get(borrower).map(|(_, n)| *n).unwrap_or(0)
    }
}

/// Whitelist gate used by `WhitelistBorrow` to allow only registered owners.
pub struct PermissionGate {
    allowed: HashSet<[u8; 32]>,
}

impl PermissionGate {
    pub fn new(allowed: impl IntoIterator<Item = [u8; 32]>) -> Self {
        Self {
            allowed: allowed.into_iter().collect(),
        }
    }

    pub fn permits(&self, borrower: &[u8; 32]) -> bool {
        self.allowed.contains(borrower)
    }

    pub fn len(&self) -> usize {
        self.allowed.len()
    }

    pub fn is_empty(&self) -> bool {
        self.allowed.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_reputation_round_trip() {
        let rep = MemoryReputation::new();
        let me = [42u8; 32];
        rep.record(me, 7500, 12);
        assert_eq!(rep.score(&me), 7500);
        assert_eq!(rep.repayment_count(&me), 12);
        let stranger = [0u8; 32];
        assert_eq!(rep.score(&stranger), 0);
    }

    #[test]
    fn gate_admits_only_listed() {
        let me = [1u8; 32];
        let you = [2u8; 32];
        let gate = PermissionGate::new([me]);
        assert!(gate.permits(&me));
        assert!(!gate.permits(&you));
    }
}

#[cfg(test)]
mod extra_tests {
    use super::*;

    #[test]
    fn gate_with_empty_initialiser_is_empty() {
        let g = PermissionGate::new(std::iter::empty::<[u8; 32]>());
        assert!(g.is_empty());
        assert_eq!(g.len(), 0);
        assert!(!g.permits(&[0u8; 32]));
    }

    #[test]
    fn gate_with_multiple_addresses_accepts_each() {
        let a = [1u8; 32];
        let b = [2u8; 32];
        let c = [3u8; 32];
        let g = PermissionGate::new([a, b, c]);
        assert_eq!(g.len(), 3);
        assert!(g.permits(&a));
        assert!(g.permits(&b));
        assert!(g.permits(&c));
        assert!(!g.permits(&[9u8; 32]));
    }

    #[test]
    fn reputation_update_overwrites_previous_score() {
        let rep = MemoryReputation::new();
        let me = [7u8; 32];
        rep.record(me, 3000, 2);
        rep.record(me, 9000, 17);
        assert_eq!(rep.score(&me), 9000);
        assert_eq!(rep.repayment_count(&me), 17);
    }
}
