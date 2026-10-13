# Evidence Delta Engine (EDE) + Unified Adaptive Engine

Deterministic **Learn** and **Apply** phases of the Unified Adaptive Engine (UAE).

## Status (Phase-1 complete)

- Sealed governance + GateEvaluated before Apply
- Authn (session tokens) + dual-control RatificationService
- CEP digest binding, fail-closed gate principal, ratify replay protection
- KernelTrace + LedgerValidator on Complete
- Formal bridge (Rust) + Lean skeleton (`sorry` OK)
- Dual-control test matrix: 16 tests

## Quick start

```bash
cargo run --example drive_cycle
cargo run --example auth_ratify
cargo test --features test-hooks --test ratify_dual_control
```

## Cycle path

```
Speculate -> Collaborate -> Learn -> Gate -> [Ratify] -> Apply -> Complete
```

## License

Proprietary – Operator Pages / internal use.

See STATUS.md for GitHub sync notes.
