# GitHub sync status

Repository: https://github.com/danpennic-sys/evidence-delta-engine

## On GitHub (this push wave)
- Cargo.toml, CI workflow, src/lib.rs, digest, orchestrator errors
- README / STATUS

## Full tree is committed locally on `main`
Includes: auth, governance/ratify, orchestrator, saga, formal, compiler, Wasm, Lean UaeKernel, dual-control tests (16).

To finish uploading the full tree from a machine with git credentials:

```bash
cd /path/to/ede
git remote add origin https://github.com/danpennic-sys/evidence-delta-engine.git
git pull origin main --allow-unrelated-histories
git push -u origin main
```
