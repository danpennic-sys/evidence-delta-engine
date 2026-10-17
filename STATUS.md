# GitHub sync status

**Repo:** https://github.com/danpennic-sys/evidence-delta-engine

## Why not full `git push`?

From this environment:
```
git push -u origin main
# fatal: could not read Username for 'https://github.com': No such device or address
```
HTTPS has no interactive credentials. File-by-file API upload is partial.

## Local (complete)

Commit: `0da678c Phase-1 hardened UAE/EDE control plane`

Remote: `origin -> https://github.com/danpennic-sys/evidence-delta-engine.git`

## On GitHub (API batches so far)

- Cargo.toml, README, STATUS, Phase-1 CI
- src/lib.rs, src/id.rs, src/timeutil.rs
- src/governance/{sealed,digest}.rs
- src/orchestrator/error.rs

## Finish from your machine (recommended)

```bash
# Option A: push local clone
cd /path/to/ede
git remote add origin https://github.com/danpennic-sys/evidence-delta-engine.git
git pull origin main --allow-unrelated-histories
git push -u origin main

# Option B: unzip source pack then push
unzip ede-source-push.zip -d evidence-delta-engine
cd evidence-delta-engine
git init && git add . && git commit -m "Phase-1 full tree"
git remote add origin https://github.com/danpennic-sys/evidence-delta-engine.git
git push -u origin main --force  # only if intentional overwrite of partial API tree
```
