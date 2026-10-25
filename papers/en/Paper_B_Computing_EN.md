# Fractal Zero-Divisor Computing — B: Generative Operators and Parallel Computing

**Author:** Vladimír Bíža  
**Research manuscript · English technical edition · 25 October 2026**  
**Scope:** Mathematics and computer science; independent of any physical theory.

## Abstract

We translate a signed Cayley–Dickson information forest into a practical algorithmic design: compute algebraically determined relationships at query time, represent inherited adjacency as a matrix-free operator, track new qualified relations in a separate frontier, and preserve noncommutative, nonassociative and nonalternative information as typed channels. We provide a resource model, PostgreSQL storage design, GPU execution plan and reproducible Python/Rust reference workflow. The design is intended for compatible computational structures, not as a universal replacement for sparse algebra, databases or consensus.

## 1. The computational primitive is a structural address

Use a typed structural key rather than allocating an object for every implied graph edge. An address includes its Cayley–Dickson level, basis indices, XOR color, orientation, relative sign, ancestry word, birth level, and relation type. A sign oracle evaluates a basis product using O(log N) stages and constant auxiliary storage in an iterative implementation.

The XOR color never replaces all four indices of a plaquette. A relation type distinguishes ZD, NZD, commutator, associator, alternator, shared incidence and birth. A mathematical key specifies algebraic structure but does not itself store arbitrary application data.

## 2. Do not store annihilating matrices; apply their operators

For an inherited component, the adjacency is A₀⊗J_m. Matrix–vector multiplication can be performed by reducing each ancestry block to a scalar, multiplying the root vector by the small sparse root adjacency, and broadcasting each root response to descendants. This has work Θ(rm+e) for r root vertices and e root edges instead of Θ(r²m²) dense work, excluding input/output storage.

The Python reference below shows the exact transformation. Crucially, it models the inherited component only; the complete zero-divisor forest also has newly born components. The speedup here is a property of matrix factorization, shared with general implicit linear operators, not an automatic breakthrough of zero-divisor algebra over all structured solvers.

\[
s_q=\sum_{v=1}^{m} x_{q,v},\quad r_p=\sum_q (A_0)_{pq}s_q,\quad y_{p,u}=r_p.
\tag{1}
\]

```python
def apply_inherited(a0, x):
    # x[p][u], p in 0..root-1, u in 0..m-1
    sums = [sum(block) for block in x]
    out = [sum(a0[p][q] * sums[q] for q in range(len(a0)))
           for p in range(len(a0))]
    return [[out[p]] * len(x[p]) for p in range(len(a0))]
```

## 3. Inherited structure versus born frontier

At each doubling we distinguish (a) an immutable root or known inherited factor, (b) the signed recursion, (c) a frontier describing newly qualified edges, and (d) optional caches. The number of newborn edges is exactly E_{2N}−4E_N, but a count is not a constructive enumerator. To materialize the frontier one must actually generate candidate address tuples or use an equivalent constructive classifier and confirm them with the plaquette predicate.

A useful API reports not just a boolean zero-product decision but provenance: which root generated the relation, its sign, its generation and the check that certified it.

## 4. Applications to AI and stateful systems

The algebraic forest is a structural index, not a pretrained cognitive model. In AI and graph-like application state we distinguish an algebraic key, recorded mutable state, and semantic payload. Only mathematically implied incidences are recomputed; actual learned parameters, observations and commitments must be persisted.

Triples preserve explicit parentheses. An associative reduction or a careless GPU kernel can destroy the information carried by nonassociativity. Krylov observability permits an exact test for which operator dimensions are absent from a selected readout. These are potential components of AI infrastructure and query planning, not evidence that the algebra alone learns tasks.

## 5. Memory hierarchy: VRAM, RAM and NVMe

Keep active sparse amplitudes, GPU batches and Krylov workspaces in VRAM; ancestry indexes, caches and the current birth frontier in RAM; and immutable roots, committed application deltas and provenance on NVMe. Avoid storing a full Cayley–Dickson multiplication table if only a small signed support is active. The memory required for actual dense state vectors still scales with the amount of state; an address oracle does not compress arbitrary user data.

## 6. GPU parallelism and deterministic execution

Partition kernels by XOR color, ancestry prefix, relation kind and active support. For inherited blocks perform reduction and root aggregation, then broadcast. For born blocks use filtered enumeration and sparse gathers. Signed basis multiplication groups naturally by XOR and the recursive sign bit; struct-of-arrays layouts permit coalesced loads. Synchronization remains necessary wherever associators or other probes couple otherwise independent blocks.

A safe deterministic pipeline preserves orientation, relative sign, provenance, and complete parenthesization. The steps below are an implementation contract rather than an empirical performance claim.

```text
stage A: select active (root, chi, ancestry, relation-kind)
stage B: classify inherited factor / born frontier
stage C: batch sign oracle + sparse algebraic operators
stage D: reduce partial responses deterministically
stage E: collect closure residual + persist qualified delta
```

## 7. Databases and generic trees

Two separate concepts should not be conflated: a source-algebra index with fully determined signed identities, and a generic hierarchical business-data index inspired by algebraic structural sharing. An arbitrary customer relationship cannot be reconstructed from XOR bits unless explicitly mapped and qualified.

The PostgreSQL example separates immutable algebraic roots from persistent delta records. Real semantic payloads are content addressed and stored transactionally. Binary canonical address keys should use byte encoding when fixed-width integer types are insufficient.

```sql
CREATE TABLE zd_root (
  root_id BIGINT PRIMARY KEY,
  level INT NOT NULL,
  chi BIGINT NOT NULL,
  scheme_version INT NOT NULL,
  immutable_hash BYTEA NOT NULL
);
CREATE TABLE zd_delta (
  root_id BIGINT REFERENCES zd_root(root_id),
  ancestry BYTEA NOT NULL,
  birth_level INT NOT NULL,
  a BIGINT NOT NULL, b BIGINT NOT NULL,
  c BIGINT NOT NULL, d BIGINT NOT NULL,
  relation_kind SMALLINT NOT NULL,
  orientation SMALLINT NOT NULL,
  payload_ref BYTEA,
  PRIMARY KEY(root_id, ancestry, a,b,c,d,relation_kind)
);
CREATE INDEX zd_delta_birth_idx ON zd_delta(birth_level, relation_kind);
```

## 8. Higher-π invariants as exact test oracles

The mathematics companion supplies exact quantities such as V_N, Φ_N, Q_N, phase-restricted Q_N and the norm-weighted compound C_N. Q_N is a mean squared norm-multiplicativity defect of independent unit source vectors and serves as a deterministic analytical target for validating a signed kernel. It is not a free performance multiplier or a proxy for intelligence.

The Fraction-based Python reference avoids Monte Carlo as a correctness oracle. A runtime may optionally collect sampled numerical estimates, but must compare those against the exact rational formula under well-defined tolerances.

```python
from fractions import Fraction
from math import comb

def edges(n):
    return 0 if n < 16 else (n-2)*(n-4)*(n-8)//16

def q(n):
    return Fraction(32*edges(n), n*n*(n+2)**2)

def q_phase(n):
    return Fraction(32*edges(n), (n-1)**2*(n+1)**2)

def compound(n):
    return Fraction(comb(n,n//2), 1) * (1+q(n))**2 / (1+q(2*n))

assert q(16) == Fraction(7,216)
assert q(32) == Fraction(315,9248)
assert compound(8) == Fraction(15120,223)
```

## 9. Reproducibility and benchmarking contract

The supplied Python scripts check the canonical zero-divisor counts and moment identities. A Rust reference implements the signed oracle and should be compiled and tested on the target machine before claims about native speed are made. Compare implementations on matching workloads and record RSS, GPU memory, p50/p99 latency, throughput, and correctness.

Fair baselines include explicit sparse representations such as CSR/GraphBLAS and optimized matrix-free/Kronecker operators. The latter can match the factorized application cost; the distinct contribution of the zero-divisor atlas is the signed generative incidence, multi-operator typing and newborn-edge genealogy.

## 10. First computing synthesis

The key engineering principle is to store qualified novelty and application payload while deriving every mathematical relation that the algebra can reconstruct. This is useful when the application has a compatible structure, not as a universal replacement for databases, consensus or access control. A zero product is not a confidentiality primitive, and rank-based recovery requires adequate independent surviving information.

## 11. Complete typed contract and invariants

Separate AlgebraKey, StorageKey and SemanticPayload. The algebraic key records convention and tower version, full signed address, root and operator type. The storage key additionally records epoch and persistence identity; payload is an independent content-addressed datum. Nothing in the signed recursion recreates an unrecorded user event.

A compiler must distinguish Mul(Mul(x,y),z) from Mul(x,Mul(y,z)). Cross-device tests validate exact signs, supports and associators separately from floating-point tolerances.

## 12. Compute kernels and cost model

Let the root have r nodes and e edges, with m=2^t descendants per root. An explicit dense adjacency stores r²m² entries. The factorized inherited matvec uses Θ(rm+e) operations and O(r) scratch beyond input/output, with no need for r²m² temporary storage. A random edge query needs the root addresses, signed predicate and ancestry; no full matrix allocation.

Newborn-edge enumeration is separate. It performs deterministic candidate generation, W=+1 classification and deduplication of signed keys. Replacing the old matrix with only a Kronecker factor computes only an inherited subgraph, not the entire source forest.

## 13. Parallel determinism and persistence

Partition work units by level, XOR color, root, ancestry prefix and channel and assign a monotonic epoch. Each unit produces an immutable delta with its source hash, operator and payload hash. Floating-point reductions need a prescribed merge order; symbolic operations may be reordered only when the particular algebraic operation permits it. Persist application events transactionally and reconstruct inherited mathematics on demand.

A cold restart validates tower-sign convention, root fingerprints and persisted provenance. Multi-channel probes preserve zero/nonzero, commutator, associator, alternator and birth responses as separate outputs; incomplete Krylov rank is reported as an unobservable complement.

## 14. Regression and experimental protocol

The explicit and factorized matvec references must produce identical outputs on identical inputs at small dimensions before benchmark curves are reported. Record algorithm name, dimension, elapsed time, memory estimate and correctness status. Do not extrapolate one device's measured latency into a universal hardware theorem.

Any learning-oriented application needs independent tests for structural correctness, persistence correctness and actual competence changes, preferably with the teaching oracle removed during evaluation. Memory compression is not by itself evidence of learning.

## 15. Conclusion

The signed Cayley–Dickson convention gives an addressable algebraic oracle; the inherited genealogy gives fast implicit operators; the birth frontier preserves genuine new edges; and higher-order probes retain information absent from a pairwise adjacency. The construction provides a concrete, testable software architecture for compatible mathematical state spaces. Its algorithms and their limitations are explicit, enabling fair reproducible comparison against contemporary sparse and structured operators.

## References

[1] V. Bíža, *Fractal Zero-Divisor Computing — A*, companion mathematical paper.

[2] G. Moreno, *Constructing zero divisors in the higher dimensional Cayley–Dickson algebras*, arXiv:math/0512517.

[3] T. A. Davis, *Algorithm 1000: SuiteSparse:GraphBLAS*, ACM Transactions on Mathematical Software.
