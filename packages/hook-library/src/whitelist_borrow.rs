//! WhitelistBorrow — only registered borrowers may open new debt.
//! Knot: lock knot. Used for KYC pools or institutional venues.

use lien_hook_runtime::{
    Hook, HookContext, HookDecision, HookFlag, HookFlags, HookMeta,
    event::LifecycleEventKind,
    permission::PermissionGate,
};

pub struct WhitelistBorrow {
    meta: HookMeta,
    pub gate: PermissionGate,
}

impl WhitelistBorrow {
    pub fn new(gate: PermissionGate) -> Self {
        let flags = HookFlags::empty()
            .with(HookFlag::BeforeBorrow)
            .with(HookFlag::MayReject);
        Self {
            meta: HookMeta {
                name: "WhitelistBorrow".into(),
                version: "1.0.0".into(),
                author: "lien-core".into(),
                flags,
                description: "Only borrowers listed in the gate may open new debt.".into(),
            },
            gate,
        }
    }
}

impl Hook for WhitelistBorrow {
    fn meta(&self) -> &HookMeta {
        &self.meta
    }

    fn evaluate(&self, ctx: &HookContext<'_>) -> HookDecision {
        if ctx.event.kind != LifecycleEventKind::BeforeBorrow {
            return HookDecision::Accept;
        }
        if self.gate.permits(&ctx.event.position.owner) {
            HookDecision::Accept
        } else {
            HookDecision::Reject(format!(
                "WhitelistBorrow: borrower not on allowlist ({} slots)",
                self.gate.len()
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lien_hook_runtime::event::{
        AdapterKind, LifecycleEvent, MarketSnapshot, PositionSnapshot,
    };

    fn evt(owner: [u8; 32]) -> LifecycleEvent {
        LifecycleEvent {
            kind: LifecycleEventKind::BeforeBorrow,
            adapter: AdapterKind::Marginfi,
            position: PositionSnapshot {
                owner,
                collateral_mint: [2; 32],
                debt_mint: [3; 32],
                collateral_amount: 1_000,
                debt_amount: 0,
                ltv_bps: 0,
                liquidation_threshold_bps: 8_000,
            },
            market: MarketSnapshot {
                slot: 0,
                timestamp: 0,
                oracle_points: vec![],
                realised_vol_bps: 0,
                utilisation_bps: 0,
            },
            payload: vec![],
        }
    }

    #[test]
    fn admits_listed() {
        let me = [1u8; 32];
        let h = WhitelistBorrow::new(PermissionGate::new([me]));
        let e = evt(me);
        let ctx = HookContext { event: &e, composition_index: 0, composition_total: 1 };
        assert_eq!(h.evaluate(&ctx), HookDecision::Accept);
    }

    #[test]
    fn rejects_unlisted() {
        let me = [1u8; 32];
        let you = [2u8; 32];
        let h = WhitelistBorrow::new(PermissionGate::new([me]));
        let e = evt(you);
        let ctx = HookContext { event: &e, composition_index: 0, composition_total: 1 };
        assert!(matches!(h.evaluate(&ctx), HookDecision::Reject(_)));
    }
}

#[cfg(test)]
mod extra_tests {
    use super::*;
    use lien_hook_runtime::event::{
        AdapterKind, LifecycleEvent, MarketSnapshot, PositionSnapshot,
    };

    fn evt(kind: LifecycleEventKind, owner: [u8; 32]) -> LifecycleEvent {
        LifecycleEvent {
            kind, adapter: AdapterKind::Marginfi,
            position: PositionSnapshot {
                owner, collateral_mint: [2; 32], debt_mint: [3; 32],
                collateral_amount: 1_000, debt_amount: 0,
                ltv_bps: 0, liquidation_threshold_bps: 8_000,
            },
            market: MarketSnapshot {
                slot: 0, timestamp: 0, oracle_points: vec![],
                realised_vol_bps: 0, utilisation_bps: 0,
            },
            payload: vec![],
        }
    }

    #[test]
    fn non_borrow_event_passes_unconditionally() {
        let me = [1u8; 32];
        let h = WhitelistBorrow::new(PermissionGate::new([me]));
        let e = evt(LifecycleEventKind::AfterDeposit, [9u8; 32]);
        let ctx = HookContext { event: &e, composition_index: 0, composition_total: 1 };
        assert_eq!(h.evaluate(&ctx), HookDecision::Accept);
    }

    #[test]
    fn empty_allowlist_rejects_everyone() {
        let h = WhitelistBorrow::new(PermissionGate::new(std::iter::empty::<[u8; 32]>()));
        let e = evt(LifecycleEventKind::BeforeBorrow, [1u8; 32]);
        let ctx = HookContext { event: &e, composition_index: 0, composition_total: 1 };
        assert!(matches!(h.evaluate(&ctx), HookDecision::Reject(_)));
    }
}
