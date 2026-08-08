# Copperfin Compatibility Corpus

This repository turns useful ideas from the old `MATCHPRG` network tools and
`LIBFUNCT.PRG` utility library into organization-neutral, testable reference
implementations for Project Copperfin and future migration projects.

The original source trees are never modified. No legacy data is copied here.

## What is included

- dependency-free Rust geodesy with great-circle distance, initial bearing,
  and spherical destination points
- planar polyline, turn-angle, and three-point radius helpers
- Dijkstra shortest paths instead of record-order link crawling
- exact Held-Karp travelling-salesperson tours for small instances
- explicitly non-optimal nearest-neighbour/2-opt tours for larger instances
- updated VFP-compatible `LIBFUNCT` and `MATCHPRG` subsets
- executable VFP/Copperfin contract programs and shared test vectors
- a small command-line runner for independent reference results

## Verify

```powershell
.\scripts\verify.ps1
.\scripts\verify-copperfin.ps1 -CopperfinRoot E:\Project-Copperfin
```

Or run the individual commands:

```powershell
cargo test --all-targets
cargo run --bin corpus-runner -- distance 0 0 0 1
cargo run --bin corpus-runner -- destination 0 0 1000 90
cargo run --bin corpus-runner -- demo-route
```

## Compatibility corpus

The VFP files under `corpus\vfp` deliberately retain familiar function names
such as `NEXTLAT`, `NEXTLONG`, `ARCLENGTH`, `DIRECTION`, and `SHORTESTPATH`.
They are rewritten, generic fixtures—not patched copies of the legacy files.

The Rust implementation is the reference oracle. Copperfin can execute the VFP
contract programs and compare their results with the Rust test vectors. The
Copperfin verification script requires an already-built Release runtime host.

See [docs/provenance.md](docs/provenance.md) for the recovery boundary and the
specific defects corrected.
