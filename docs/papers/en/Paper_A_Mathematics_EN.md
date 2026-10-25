# Fractal Zero-Divisor Computing — A: Signed Cayley–Dickson Information Forest and Higher-π Invariants

**Author:** Vladimír Bíža  
**Research manuscript · English technical edition · 25 October 2026**  
**Scope:** Mathematics and computer science; independent of any physical theory.

## Abstract

We develop a recursively addressable, signed multi-channel atlas of two-term zero-divisor relations in the real Cayley–Dickson tower. An exact XOR-and-plaquette predicate classifies selected annihilating pairs; inherited families exhibit fourfold lifts, mutually annihilating subspaces and tensor-factorized adjacency, while new relations are born at every doubling. A separate algebraic phase refinement defines a source period converging to π and supplies closed-form volume and norm-defect invariants. We specify the exact domain of each count, distinguish pairwise edges from higher-order information, and provide reference validation contracts.

## 1. Canonical Cayley–Dickson source algebra

We use the real positive-definite, non-split Cayley–Dickson tower with the multiplication convention displayed below. The basis multiplication is a signed XOR permutation. The sign is not a lookup-table primitive: its recurrence follows from doubling and can be evaluated using logarithmically many stages. The convention is part of the reproducibility contract; switching conventions without an explicit isomorphism invalidates comparison of signed witnesses.

The tower becomes nonassociative at the octonions and, beginning with the sedenions, loses alternativity and develops nontrivial zero divisors. Here a zero divisor is a nonzero element having zero product with another nonzero element, not a floating-point underflow.

\[
(a,b)(c,d)=(ac-\bar d b,\;da+b\bar c).
\tag{1}
\]

\[
e_a e_b=\varepsilon_k(a,b)e_{a\oplus b},\qquad \varepsilon_k(a,b)\in\{-1,1\}.
\tag{2}
\]

\[
\begin{aligned}
\varepsilon_{k+1}(a,b)&=\varepsilon_k(a,b),\\
\varepsilon_{k+1}(a,N+b)&=\varepsilon_k(b,a),\\
\varepsilon_{k+1}(N+a,b)&=\eta(b)\varepsilon_k(a,b),\\
\varepsilon_{k+1}(N+a,N+b)&=-\eta(b)\varepsilon_k(b,a).
\end{aligned}\tag{3}
\]

## 2. Four-address zero-product predicate

An assessor is an unordered pair of distinct nonzero basis addresses, together with a choice of sign on the corresponding two-term imaginary vector. Two disjoint assessors can cancel their product only if their XOR colors coincide. Expanding the signed basis rule produces two coefficients on different XOR addresses; both must vanish. Their compatibility is the four-sign plaquette invariant W.

**Theorem 1 (complete two-term predicate).** For disjoint assessors with equal XOR color, there exist nonzero signed directions whose product vanishes if and only if W=+1. The relative signs must satisfy the displayed condition. **Proof.** Equate the two coefficients of the product to zero, multiply the conditions to obtain W=+1, and back-substitute to select the relative sign. In every implementation the shared-color and disjointness checks precede the plaquette test.

\[
xy=(\varepsilon_{ac}+\sigma\tau\varepsilon_{bd})e_{a\oplus c}+(\tau\varepsilon_{ad}+\sigma\varepsilon_{bc})e_{a\oplus d}.
\tag{4}
\]

\[
W(a,b;c,d)=\varepsilon_{ac}\varepsilon_{bd}\varepsilon_{ad}\varepsilon_{bc}.
\tag{5}
\]

## 3. The K16 root, its spectrum, and the nonzero complement

At K16 the seven active colors are 9 through 15. Each has six unsigned assessors arranged into a complete tripartite graph K(2,2,2), with three antipodal nonedge pairs. The graph has 12 edges, and hence there are 84 zero-product assessor edges. There are 315 candidate assessor pairs and 231 nonzero complement edges in the designated atlas. These counts concern two-term assessors, not all zero divisors in the continuous algebraic variety.

The explicit zero-product witness below also vanishes when the factors are reversed. Because a binary graph retains only an edge label, it must not be mistaken for a representation of all signed or higher-order products.

\[
|V|=6,\quad |E|=12,\quad\operatorname{spec}(A)=\{4,-2,-2,0,0,0\}.
\tag{6}
\]

\[
(e_2+e_{11})(e_4-e_{13})=0=(e_4-e_{13})(e_2+e_{11}).
\tag{7}
\]

## 4. Fourfold inheritance and mutually annihilating subspaces

**Theorem 2 (four-lift).** Let x and y be orthogonal, purely imaginary elements of K_N with xy=yx=0. Doubling produces four mixed descendants whose products vanish in either order. **Proof.** Substitute all four combinations of (x,0), (0,x), (y,0), and (0,y) into the Cayley–Dickson multiplication law. Each component is an old zero product or its conjugate. Induct on the number of doublings.

This does more than clone edges. By bilinearity, the descendants span two independent subspaces each of dimension 2^t, and every cross-product between those subspaces vanishes. Passing to nonzero directions modulo real scaling yields a projective product variety. For t=7 the two 128-dimensional spaces have 16,384 basis-pair combinations; both product orientations yield 32,768 vanishing products.

\[
x_0=(x,0),\quad x_1=(0,x),\quad y_0=(y,0),\quad y_1=(0,y).
\]

\[
V_x^{(t)}V_y^{(t)}=V_y^{(t)}V_x^{(t)}=\{0\},\quad
\dim V_x^{(t)}=\dim V_y^{(t)}=2^t.
\tag{8}
\]

\[
\mathbb{RP}^{2^t-1}\times\mathbb{RP}^{2^t-1}\subseteq\mathscr Z_{k+t}^{\mathrm{proj}}.
\tag{9}
\]

## 5. Factorized spectra and the genealogical boundary

**Theorem 3 (inherited-component factorization).** For an unchanged root color, each edge lifts to a complete bipartite four-edge block while nonedges stay nonedges. Consequently its adjacency is the Kronecker product of the root adjacency and an all-ones matrix. This statement applies to the inherited component, not to every newly born edge in the full tower.

The spectrum follows from the spectrum of K(2,2,2) and the all-ones factor: one positive and two negative nonzero eigenvalues, scaled by 2^t, with a growing zero eigenspace. The normalized nonzero spectrum remains fixed. The sequence of ancestry words has a Cantor-type boundary whose tripartite graphon is the limit of this particular inherited component.

\[
A_{\chi,k+t}=A_{\chi,k}\otimes J_{2^t},\qquad J_m=\mathbf1_m\mathbf1_m^T.
\tag{10}
\]

\[
\operatorname{spec}(A_t)=\{4\cdot 2^t,-2\cdot2^t,-2\cdot2^t,0^{\times(6\cdot2^t-3)}\}.
\tag{11}
\]

## 6. Newborn edges and the asymptotic 50/50 law

The exact count of unsigned two-term assessor edges at dimension N is E_N below. The lift contributes exactly four edges for each old edge; all remaining edges are newly born rather than descendants of a fixed root.

**Theorem 4 (birth budget).** The newborn count equals E_{2N}−4E_N, with the closed form and ratio displayed below. **Proof.** Substitute the polynomial edge-count formula at N and 2N and simplify symbolically. The fraction of newborn edges converges to one half from above. This prevents representing the complete information forest as a single Kronecker inflation of the K16 root. The census is a structural statement about the selected assessor atlas, not about arbitrary graphs.

\[
E_N=\frac{(N-2)(N-4)(N-8)}{16},\quad N\ge16.
\tag{12}
\]

\[
B_{2N}=E_{2N}-4E_N=\frac{(N-2)(N-4)(N+6)}4
\tag{13}
\]

\[
\frac{B_{2N}}{E_{2N}}=\frac{N+6}{2(N-1)}=\frac12+\frac7{2(N-1)}\longrightarrow\frac12.
\tag{14}
\]

| N | \(E_N\) | \(B_N\) |
|---:|---:|---:|
|16|84|84|
|32|1260|924|
|64|13020|7980|
|128|117180|65100|
|256|992124|523404|
|512|8161020|4192524|


## 7. Multiple algebraic information channels

We preserve at least seven distinguishable channels: zero product (ZD), nonzero product (NZD), shared incidence, commutator, associator, alternator, and birth. A zero pairwise product does not imply that all higher products involving a third input also vanish.

For a concrete sedenion witness, take x=e_2+e_11, y=e_4−e_13, and z=e_1. The direct product xy is zero but the associator (xz)y−x(zy) is −4e_7. An index that records only the pairwise zero-product graph irrevocably drops that higher-order distinction. Observability may also be analyzed by Krylov ranks over selected operator probes; application-level semantics must be supplied independently of the algebra.

## 8. Higher-π: a source period before geometric volumes

The first associative phase plane is the K2 subalgebra generated by a unit imaginary Y. A real scalar character of order three is impossible in K1 but an algebraic cubic root of unity exists in K2. The integer lattice quotient Λ24/3Λ24 provides a natural finite group on which such a character acts; it is an algebraic construction, not a claim about physical spacetime.

**Theorem 8.1 (source-period reconstruction).** Let the phase roots and ternary refinements be defined as shown in the equations. Their inscribed and tangent half-perimeters form algebraic lower and upper approximants whose difference decays quadratically with cycle length. Both converge to the classical constant π. **Proof.** Use u_{r+1}³=u_r, identify |1−u_r| as the corresponding polygon chord, establish the familiar polygon bounds, and take the common limit. The numerical value π is not supplied to the recursion. The source-defined limit is identified geometrically only after its algebraic construction.

The cyclic integer Laplacian supplies a separate spectral path to the same period. Every unit imaginary vector of K_N generates a complex two-dimensional associative plane. The number π remains the same; what changes with N is the dimension of the family of planes and the volumetric invariants. Gaussian integration gives the unit-ball and phase-plane formulas, and their ratios yield the central binomial fusion coefficients.

\[
\omega=\frac{-1+\sqrt3Y}{2},\qquad \omega^3=1,\qquad 1+\omega+\omega^2=0,\qquad \chi_y(x)=\omega^{\langle x,y\rangle\bmod3}.
\tag{15a}
\]

\[
\Pi_r^-=\frac{N_r}{2}\sqrt{\lambda_r},\qquad
\Pi_r^+=\frac{N_r\sqrt{\lambda_r}}{2\sqrt{1-\lambda_r/4}},\qquad
\boxed{\Pi_{\mathrm{src}}:=\lim_{r\to\infty}\Pi_r^-}.\tag{15b}
\]

\[
\boxed{\lim_{n\to\infty}\frac{F_n}{2}\sqrt{\lambda_+(L_{F_n})}=\Pi_{\mathrm{src}}=\pi},\tag{15c}
\]

\[
\Pi_u=\Pi_{\mathrm{src}},\qquad\mathfrak F_N=S^{N-2}/\{u\sim-u\}\cong\mathbb{RP}^{N-2}.
\tag{15}
\]

\[
V_N=\frac{\Pi_{\mathrm{src}}^{N/2}}{\Gamma(N/2+1)},\qquad
\Phi_N=\frac{\Pi_{\mathrm{src}}^{(N-1)/2}}{\Gamma((N-1)/2)}.
\tag{16}
\]

\[
\frac{V_N V_M}{V_{N+M}}=\binom{(N+M)/2}{N/2},\qquad
\mathcal B_N=\frac{V_N^2}{V_{2N}}=\binom N{N/2}.
\tag{17}
\]

## 9. Norm defect from the same zero-product atlas

Symmetrized cross-terms of left multiplication define matrices B_ij. Their Frobenius trace contraction is supported exactly on the same positive plaquettes that qualify zero-product assessor edges. Counting orientations gives the first identity below; linearized alternativity defects then yield the second.

A pair of independent uniform unit vectors has fourth moments given by the standard Kronecker-delta contraction. Substitution gives a closed-form mean-square defect of norm multiplicativity Q_N, together with a related phase-restricted moment. These vanish at dimensions 2, 4 and 8, turn positive at 16, and decay as 2/N at large dimensions. The explicitly tabulated rational checks are reference invariants for a correct signed implementation.

Define a norm-weighted volume as an algebraic quantity, rather than claiming that the ordinary Euclidean volume itself changes. Its fusion ratio combines the same ball-volume binomial coefficient with the rational defect moments. This is the intended meaning of the higher-π compound invariant.

\[
B_{ij}=\frac12(L_i^TL_j+L_j^TL_i),\quad i<j.
\tag{18}
\]

\[
\sum_{i<j}\operatorname{tr}(B_{ij}^2)=4E_N.
\tag{19}
\]

\[
\sum_{a,b,c}\|T_{abc}\|^2=32E_N.
\tag{20}
\]

\[
\mathbb E[x_ix_jx_ax_b]=\frac{\delta_{ij}\delta_{ab}+\delta_{ia}\delta_{jb}+\delta_{ib}\delta_{ja}}{N(N+2)}
\]

\[
Q_N=\mathbb E(\|xy\|^2-1)^2
=\frac{32E_N}{N^2(N+2)^2}
=\frac{2(N-2)(N-4)(N-8)}{N^2(N+2)^2}.
\tag{21}
\]

\[
Q_N^{\mathrm{phase}}=\frac{32E_N}{(N-1)^2(N+1)^2}.
\tag{22}
\]

\[
\mathcal C_N=\frac{\mathcal P_N^2}{\mathcal P_{2N}}=\binom N{N/2}\frac{(1+Q_N)^2}{1+Q_{2N}},
\tag{23}
\]

## 10. Algebraic synthesis

The construction is a signed, typed and locally reconstructible information forest rather than a static graph or a list of special constants. It combines an exact address oracle, recursively inherited annihilating subspaces, a separate newborn-edge budget, analyzable spectral limits, and exact moments of norm nonmultiplicativity. Its usefulness for computing rests on preserving all these layers separately.

## 11. Object boundaries and inverse addressing

We distinguish four carriers: basis addresses; signed two-term assessors; their selected unsigned zero-product graph; and the continuous variety of all zero divisors. Numerical edge counts describe only the third. Projective subspaces furnish explicit subsets of the fourth. Conflating them would produce wildly misleading scalability figures.

The canonical signed key stores level, XOR color, the two sorted assessor pairs, relative signs, orientation and ancestry. Lexicographic canonicalization deduplicates a chosen root family but need not establish global identity across different roots or equivalent geometric directions. A product of at least three factors must carry its full parenthesization.

**Proposition 5 (family addressing).** A fixed root, depth t, and two t-bit words uniquely determine inherited descendants using the two Cayley–Dickson embeddings. All their cross-products vanish. **Proof.** Induct on ancestry length and apply the four-lift at each step.

\[
\mathsf Z=(k,\chi,\{a,b\},\{c,d\},\sigma,\tau,\text{orientation},\text{ancestry}),\quad a\oplus b=c\oplus d=\chi.
\]

## 12. Complete transition calculus for higher-π invariants

Integrate the Gaussian in Cartesian coordinates and, independently, in radial coordinates. Equating the results gives the unit-sphere surface area, hence the volume V_N after division by N. Antipodal identification halves the appropriate sphere measure to define Φ_N. Integer-dimensional Gamma values reduce the volume-fusion identity to a binomial coefficient.

The norm-weighted volume P_N is a defined compound invariant, not a revision of Euclidean measure. Using the same edge-count polynomial in the moment formula yields the complete hierarchy Q_N and C_N.

**Proposition 6 (asymptotics).** Expansion of the exact rational expression for Q_N gives 2/N−36/N²+O(N⁻³); for the phase moment the leading term is 2/N. The displayed series fixes the coefficients without fitting or simulation.

\[
\int_{\mathbb R^N}e^{-\|x\|^2}\,dx=\Pi_{\mathrm{src}}^{N/2}
=\operatorname{area}(S^{N-1})\,\frac{\Gamma(N/2)}2.
\]

\[
\frac{V_NV_M}{V_{N+M}}=\frac{(n+m)!}{n!m!},\qquad
\mathcal B_N=\frac{N!}{((N/2)!)^2}.
\]

\[
Q_N=\frac2N-\frac{36}{N^2}+O(N^{-3}),\qquad
Q_N^{\mathrm{phase}}=\frac2N+O(N^{-2}).
\]

\[
\boxed{Q_N=\frac2N-\frac{36}{N^2}+O(N^{-3})}.
\]

## 13. Reproducibility contract

A conforming reference implementation verifies signed XOR multiplication, unit-square signs, the explicit K16 zero-product witness, the K16 84/315 counts, K32 and K64 census, equality of explicit and Kronecker-inherited operator application, and the exact rational invariants. Signed products of general sparse vectors require bilinear accumulation. Each system should run its own algebraic tests before it reports throughput.

## 14. Final mathematical statement

At every level k≥4 of the standard nonsplit real Cayley–Dickson tower, a uniform signed-address oracle exists. Its designated two-term assessor atlas has a fully specified predicate, inherited annihilator families, and a separate census of genuinely new edges. The same atlas determines rational spherical norm moments and their higher-π compounds. The infinite structure is described as a compatible sequence of finite computations, not as an infinite matrix stored in memory.

## References

[1] G. Moreno, *The zero divisors of the Cayley–Dickson algebras over the real numbers*, arXiv:q-alg/9710013.

[2] R. P. C. de Marrais, *The 42 Assessors and the Box-Kites they fly*, arXiv:math/0011260.

[3] G. Moreno, *Constructing zero divisors in the higher dimensional Cayley–Dickson algebras*, arXiv:math/0512517.
