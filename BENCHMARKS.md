# Benchmark specification (fair modern baselines)

**English** · [Čeština](BENCHMARKS_CS.md)

**Do not compare only with a dense adjacency matrix.** Report at least these methods for the same exact workload:

- **CSR / GraphBLAS**: build and apply the qualified sparse adjacency, with correct conventions and query semantics.
- **Structured / matrix-free / Kronecker operator**: retain A₀ and use block sums instead of materializing inherited Aₜ.
- **Fractal ZD signed oracle**: generate certified ZD edges, sign, birth, and higher-order probes; apply the same inherited operator where appropriate.

Separate workloads: pair classification throughput, inherited matvec, birth-frontier enumeration, signed sparse products, associator/alternator, memory on K16/32/64/2048, persist/reload, and recovery with/without redundant observables.

Report elapsed p50/p99; warm-up method; CPU/GPU model; OS; compiler/build flags; peak RSS/VRAM; correctness counts; datasets and code commit hash. Compare operations of equivalent meaning. A pairwise adjacency cannot express an associator, but hypergraphs and other higher-order frameworks can—compare representation, code complexity and verified results, not marketing slogans.

`src/python/benchmark.py` is a **small educational reference**, not a comprehensive SOTA benchmark. A competitive benchmark suite is a separate research deliverable.
