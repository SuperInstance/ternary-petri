# ternary-petri

Ternary Petri nets: place-transition dynamics on tokens drawn from {-1, 0, +1}. Enables reachability analysis, conflict detection, deadlock checking, and liveness verification for concurrent ternary agent systems.

## Why It Matters

Petri nets are the mathematical foundation for modeling concurrent, distributed, and asynchronous systems. By restricting tokens to ternary values {-1, 0, +1}, we gain:

- **Trinary semantics**: each place can represent positive evidence (+1), negative evidence (-1), or neutral/unknown (0)
- **State-space compactness**: ternary places have exactly 3 states, making exhaustive reachability feasible for small nets
- **Conservation analysis**: ternary clamping ensures transitions preserve the {-1, 0, +1} invariant automatically
- **Concurrent reasoning**: model conflicts, deadlocks, and liveness for agent coordination protocols

Applications include workflow verification, distributed protocol analysis, ternary logic circuit verification, and multi-agent resource allocation.

## How It Works

### Petri Net Structure

A Petri net is a bipartite graph of **places** (holding tokens) and **transitions** (consuming and producing tokens).

**Transition** structure (fixed 4-input/4-output for cache efficiency, ≤16 bytes):

$$T = (\mathbf{I}, \mathbf{O}, \theta)$$

where $\mathbf{I} \in \{-1,0,+1\}^4$ is the input vector, $\mathbf{O} \in \{-1,0,+1\}^4$ is the output vector, and $\theta$ is the firing threshold.

### Firing Rule

A transition is **enabled** if every input place $i$ with $I_i \neq 0$ satisfies $P_i \geq \theta$:

$$\text{enabled}(T) = \forall i: I_i \neq 0 \implies P_i \geq \theta$$

When fired, each place is updated:

$$P_i' = \text{clamp}(P_i - I_i + O_i, \;-1, \;+1)$$

The clamp ensures ternary invariant preservation.

**Complexity:** O(1) per transition firing (4 inputs, 4 outputs — fixed size).

### Reachability

BFS from initial marking through all fireable transitions:

$$\text{Reach}(M_0) = \{M : M_0 \to^* M\}$$

where $\to^*$ is the reflexive-transitive closure of the firing relation.

**Complexity:** O($3^n \cdot |T| \cdot n$) worst case for $n$ places — but typically much smaller because many markings are unreachable. Deduplication via hash set prevents revisiting states.

### Conflict Set

Transitions that share input places and are simultaneously enabled — firing one may disable others:

$$\text{Conflict}(M) = \{i : T_i \text{ is enabled at } M\}$$

### Deadlock

A marking where **no** transition can fire:

$$\text{Deadlock}(M) = \forall i: \neg\text{enabled}(T_i, M)$$

### Liveness

A transition is **live** if it can eventually fire from every reachable marking. Approximate check: simulate $k$ steps and verify all transitions fire at least once.

$$\text{Live}(M_0, k) = |\{i : T_i \text{ fires within } k \text{ steps}\}| = |T|$$

## Quick Start

```rust
use ternary_petri::*;

// Define transitions: move token from place 0 to place 1
let t1 = Transition::new_transition([1, 0, 0, 0], [0, 1, 0, 0], 1);
let t2 = Transition::new_transition([0, 1, 0, 0], [1, 0, 0, 0], 1);

// Initial marking: token in place 0
let places = [1, 0, 0, 0];

// Fire transition
let mut state = places;
assert!(t1.fire(&mut state)); // state becomes [0, 1, 0, 0]
assert_eq!(state[0], 0);
assert_eq!(state[1], 1);

// Reachability
let reachable = reachable(&[1, 0, 0, 0], &[t1, t2], 10);

// Conflict detection
let conflicts = conflict_set(&[t1, t2], &places);

// Deadlock check
assert!(!deadlock(&[1, 0, 0, 0], &[t1]));

// Liveness (two transitions cycle tokens back and forth)
assert!(live(&[1, 0, 0, 0], &[t1, t2], 10));
```

## API

| Type / Function | Description |
|---|---|
| `Transition::new_transition(inputs, outputs, threshold)` | Create a Petri net transition |
| `.enabled(places) → bool` | Check if transition can fire |
| `.fire(places) → bool` | Fire transition, return success |
| `reachable(from, transitions, steps) → Vec<Vec<i8>>` | All reachable markings |
| `conflict_set(transitions, places) → Vec<usize>` | Indices of simultaneously enabled transitions |
| `deadlock(places, transitions) → bool` | No transition can fire |
| `live(places, transitions, steps) → bool` | All transitions can eventually fire |

## Architecture Notes

Ternary Petri nets enforce the **γ + η = C** conservation identity through their clamping mechanism. Each place can hold at most one unit of constructive mass (+1 = γ) or inhibitory mass (-1 = η). The total conserved quantity $C$ is bounded by the number of non-zero places. Transitions rearrange the distribution of γ and η across places while the clamping to {-1, 0, +1} automatically preserves the ternary invariant.

The neutral state (0) is essential: it represents an "empty" place that absorbs both positive and negative token flows without overflow. This makes ternary Petri nets inherently safe from integer overflow issues that plague classical (unbounded) Petri nets, while still being expressive enough to model nontrivial concurrent protocols.

The 4-place transition structure (16 bytes total) was chosen for cache-line efficiency: a single transition fits in a single L1 cache line with room for metadata, enabling high-throughput simulation of large nets.

## References

- Petri, C. A. (1962). *Kommunikation mit Automaten.* PhD thesis, Universität Hamburg.
- Murata, T. (1989). *Petri Nets: Properties, Analysis and Applications.* Proc. IEEE, 77(4).
- Reisig, W. (2013). *Understanding Petri Nets: Modeling Techniques, Analysis Methods, Case Studies.* Springer.
- Jensen, K. & Kristensen, L. M. (2009). *Coloured Petri Nets.* Springer.

## License

MIT
