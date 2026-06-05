#![forbid(unsafe_code)]

/// Petri net dynamics on ternary tokens (-1, 0, +1).
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transition {
    pub inputs: [i8; 4],
    pub outputs: [i8; 4],
    pub threshold: i8,
}

impl Transition {
    pub fn new_transition(inputs: [i8; 4], outputs: [i8; 4], threshold: i8) -> Self {
        Self { inputs, outputs, threshold }
    }

    pub fn enabled(&self, places: &[i8]) -> bool {
        for i in 0..4 {
            if self.inputs[i] == 0 { continue; }
            if i >= places.len() { return false; }
            if places[i] < self.threshold { return false; }
        }
        true
    }

    pub fn fire(&self, places: &mut [i8]) -> bool {
        if !self.enabled(places) { return false; }
        for i in 0..4 {
            if self.inputs[i] != 0 && i < places.len() {
                places[i] -= self.inputs[i];
                places[i] = places[i].clamp(-1, 1);
            }
            if self.outputs[i] != 0 && i < places.len() {
                places[i] += self.outputs[i];
                places[i] = places[i].clamp(-1, 1);
            }
        }
        true
    }
}

pub fn reachable(from: &[i8], transitions: &[Transition], steps: usize) -> Vec<Vec<i8>> {
    let mut seen: HashSet<Vec<i8>> = HashSet::new();
    seen.insert(from.to_vec());
    let mut current: Vec<Vec<i8>> = vec![from.to_vec()];
    for _ in 0..steps {
        let mut next = Vec::new();
        for state in &current {
            for t in transitions {
                let mut s = state.clone();
                if t.fire(&mut s) && !seen.contains(&s) {
                    seen.insert(s.clone());
                    next.push(s);
                }
            }
        }
        current = next;
        if current.is_empty() { break; }
    }
    seen.into_iter().collect()
}

pub fn conflict_set(transitions: &[Transition], places: &[i8]) -> Vec<usize> {
    transitions.iter().enumerate()
        .filter(|(_, t)| t.enabled(places))
        .map(|(i, _)| i)
        .collect()
}

pub fn deadlock(places: &[i8], transitions: &[Transition]) -> bool {
    transitions.iter().all(|t| !t.enabled(places))
}

pub fn live(places: &[i8], transitions: &[Transition], steps: usize) -> bool {
    let mut fired: HashSet<usize> = HashSet::new();
    let mut state = places.to_vec();
    for _ in 0..steps {
        for (i, t) in transitions.iter().enumerate() {
            let mut s = state.clone();
            if t.fire(&mut s) {
                fired.insert(i);
                state = s;
            }
        }
        if fired.len() == transitions.len() { return true; }
    }
    fired.len() == transitions.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_transition() {
        let t = Transition::new_transition([1, 0, 0, 0], [0, 1, 0, 0], 1);
        assert_eq!(t.inputs, [1, 0, 0, 0]);
        assert_eq!(t.outputs, [0, 1, 0, 0]);
        assert_eq!(t.threshold, 1);
    }

    #[test]
    fn test_enabled_yes() {
        let t = Transition::new_transition([1, 0, 0, 0], [0, 1, 0, 0], 1);
        let places = [1, 0, 0, 0];
        assert!(t.enabled(&places));
    }

    #[test]
    fn test_enabled_no() {
        let t = Transition::new_transition([1, 0, 0, 0], [0, 1, 0, 0], 1);
        let places = [0, 0, 0, 0];
        assert!(!t.enabled(&places));
    }

    #[test]
    fn test_fire_success() {
        let t = Transition::new_transition([1, 0, 0, 0], [0, 1, 0, 0], 1);
        let mut places = [1, 0, 0, 0];
        assert!(t.fire(&mut places));
        assert_eq!(places[0], 0);
        assert_eq!(places[1], 1);
    }

    #[test]
    fn test_fire_fail() {
        let t = Transition::new_transition([1, 0, 0, 0], [0, 1, 0, 0], 1);
        let mut places = [0, 0, 0, 0];
        assert!(!t.fire(&mut places));
    }

    #[test]
    fn test_fire_clamps() {
        let t = Transition::new_transition([1, 0, 0, 0], [1, 0, 0, 0], 1);
        let mut places = [1, 0, 0, 0];
        t.fire(&mut places);
        // places[0] was 1, minus 1 = 0, plus output 1 = 1
        assert_eq!(places[0], 1);
    }

    #[test]
    fn test_reachable_basic() {
        let t = Transition::new_transition([1, 0, 0, 0], [0, 1, 0, 0], 1);
        let from = [1, 0, 0, 0];
        let states = reachable(&from, &[t], 5);
        assert!(states.len() >= 2); // at least original + fired state
    }

    #[test]
    fn test_reachable_deadlock() {
        let t = Transition::new_transition([1, 0, 0, 0], [0, 1, 0, 0], 1);
        let from = [0, 0, 0, 0];
        let states = reachable(&from, &[t], 5);
        assert_eq!(states.len(), 1);
    }

    #[test]
    fn test_conflict_set() {
        let t1 = Transition::new_transition([1, 0, 0, 0], [0, 1, 0, 0], 1);
        let t2 = Transition::new_transition([1, 0, 0, 0], [0, 0, 1, 0], 1);
        let places = [1, 0, 0, 0];
        let conflicts = conflict_set(&[t1, t2], &places);
        assert_eq!(conflicts, vec![0, 1]);
    }

    #[test]
    fn test_conflict_set_empty() {
        let t = Transition::new_transition([1, 0, 0, 0], [0, 1, 0, 0], 1);
        let places = [0, 0, 0, 0];
        let conflicts = conflict_set(&[t], &places);
        assert!(conflicts.is_empty());
    }

    #[test]
    fn test_deadlock_true() {
        let t = Transition::new_transition([1, 0, 0, 0], [0, 1, 0, 0], 1);
        let places = [0, 0, 0, 0];
        assert!(deadlock(&places, &[t]));
    }

    #[test]
    fn test_deadlock_false() {
        let t = Transition::new_transition([1, 0, 0, 0], [0, 1, 0, 0], 1);
        let places = [1, 0, 0, 0];
        assert!(!deadlock(&places, &[t]));
    }

    #[test]
    fn test_live_yes() {
        let t1 = Transition::new_transition([1, 0, 0, 0], [0, 1, 0, 0], 1);
        let t2 = Transition::new_transition([0, 1, 0, 0], [1, 0, 0, 0], 1);
        let places = [1, 0, 0, 0];
        assert!(live(&places, &[t1, t2], 10));
    }

    #[test]
    fn test_live_no() {
        let t = Transition::new_transition([1, 0, 0, 0], [0, 1, 0, 0], 1);
        let places = [1, 0, 0, 0];
        // Only one transition, should be trivially live
        assert!(live(&places, &[t], 1));
    }

    #[test]
    fn test_struct_size() {
        assert!(std::mem::size_of::<Transition>() <= 16);
    }
}
