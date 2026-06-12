# Changelog

The format is loosely based on [Keep a Changelog](https://keepachangelog.com/) but written in plain prose because the project is small.

## 0.1.0 — 2026-06-12

First public tag.

- Anchor 0.31 hook executor program with Pool / Composition / HookListing PDAs and four instructions (`register_pool`, `install_composition`, `update_composition`, `run_composition`, `publish_hook`).
- Hook runtime crate (`lien-hook-runtime`) with the lifecycle event types, `Composition` + `CompositionBuilder`, `Simulator`, and the `Hook` trait.
- Six standard hooks (`lien-hook-library`): `DynamicLTV`, `TimeTriggerLiq`, `WhitelistBorrow`, `AntiMEVLiq`, `AutoHedge`, `ReputationRate`. Each ships with unit tests covering accept and reject paths.
- Adapters for Marginfi v2, Kamino Lend, and Solend that normalise pool state into the shared `LifecycleEvent` shape.
- TypeScript SDK (`@liens-fi/sdk`) with `Composition`, `ExecutorClient`, deterministic `simulate()`, and helpers for every standard hook.
- `lien-cli` (`npm i -g lien-cli`) with `list`, `simulate`, `create`, `deploy` (plan-only — does not broadcast), and `action` (writes a GitHub Actions workflow).
- VS Code extension (`liens-fi.lien-vscode`) with three commands: Open Hook Designer, Simulate Current Composition, Show Deploy Plan.
- Docs: `docs/architecture.md`, `docs/hooks-spec.md`, `docs/security.md`.
- CI workflow at `.github/workflows/ci.yml`.

## Notes

The executor program is not yet deployed to mainnet. The placeholder pubkey in `declare_id!` is replaced at deploy time. The `lien deploy` command only prints the deploy plan; the actual `anchor deploy` is operator-driven.
