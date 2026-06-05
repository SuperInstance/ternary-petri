# ternary-petri

**Petri nets with ternary tokens. Concurrent workflows where every place holds {-1, 0, +1}.**

Petri nets are the mathematician's workflow engine. You have *places* (holding tokens) and *transitions* (consuming and producing tokens). When a transition's inputs are satisfied, it *fires* — consuming input tokens and producing output tokens. The beauty is concurrency: multiple transitions can fire simultaneously, modeling parallel processes without explicit synchronization.

This crate implements Petri nets where every place holds a ternary token: `-1`, `0`, or `+1`. Transitions consume specific input values and produce specific output values. Places are clamped to {-1, 0, +1} after each firing — no accumulation beyond the ternary range.

## What's Inside

- **`Transition`** — defines inputs, outputs, and threshold. `enabled(places)` checks if it can fire. `fire(places)` executes it
- **`reachable(from, transitions, steps)`** — compute all states reachable from a starting configuration in N steps
- **`deadlock(state, transitions)`** — check if no transition can fire (system is stuck)
- **`liveness(transitions)`** — check if all transitions can eventually fire (system is fair)
- **`fire_all(state, transitions)`** — fire all enabled transitions simultaneously (maximal parallelism)

## Quick Example

```rust
use ternary_petri::*;

// A simple workflow: consume +1 from place 0, produce -1 at place 1
let produce = Transition::new_transition(
    [1, 0, 0, 0],  // consume +1 from place 0
    [0, -1, 0, 0], // produce -1 at place 1
    0,             // threshold
);

// System state: [place0, place1, place2, place3]
let mut places = [1, 0, 0, 0]; // +1 at place 0

// Fire the transition
assert!(produce.fire(&mut places));
assert_eq!(places, [0, -1, 0, 0]); // consumed +1, produced -1

// Find all reachable states
let states = reachable(&[1, 0, 0, 0], &[produce], 5);
// [[1,0,0,0], [0,-1,0,0]] — two reachable states

// Check for deadlock
if deadlock(&places, &[produce]) {
    println!("System is stuck!"); // no transition can fire
}
```

## The Deeper Truth

**Ternary Petri nets model bounded concurrent systems.** Standard Petri nets allow unbounded token accumulation — a place can hold any number of tokens. Ternary Petri nets are *inherently bounded*: every place is clamped to {-1, 0, +1}. This makes the state space finite and enumerable: for N places, there are exactly 3^N possible states. This means reachability, deadlock, and liveness are all *decidable* — you can compute them exactly, not approximate.

The ternary constraint also forces a design discipline: you can't solve problems by "just adding more tokens." You have to design transitions that work within the three-value constraint. This leads to cleaner, more honest models — every place truly means something, because it can't be used as an infinite buffer.

**Use cases:**
- **Workflow engines** — model business processes with ternary state
- **Protocol verification** — prove properties of communication protocols
- **Concurrent programming** — model synchronization patterns
- **Biological systems** — gene regulatory networks with ternary activation
- **Education** — the simplest Petri net formalism with decidable analysis

## See Also

- **ternary-sync** — synchronization primitives for concurrent ternary systems
- **ternary-consensus** — group decision-making (Petri nets can model consensus protocols)
- **ternary-automata** — finite state machines (simpler than Petri nets, no concurrency)
- **ternary-pipeline** — staged processing (a special Petri net topology)

## Install

```bash
cargo add ternary-petri
```

## License

MIT
