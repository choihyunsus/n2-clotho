// Analyze reachability and advisory terminal-state naming.
use super::*;
use definition::violation;

pub(super) fn check(machine: &StateMachine) -> Vec<ContractViolation> {
    let mut issues = Vec::new();
    let reachable = reachable(machine);
    for state in &machine.states {
        if !reachable.contains(state) {
            issues.push(violation(&machine.name, ViolationType::UnreachableState,
                format!("상태 '{state}'에 도달할 수 있는 전이가 없습니다")));
        }
        if !machine.transitions.iter().any(|t| &t.from == state) && !likely_terminal(state) {
            issues.push(violation(&machine.name, ViolationType::DeadlockDetected,
                format!("상태 '{state}'에서 나가는 전이가 없습니다 (데드락 가능)")));
        }
    }
    issues
}

fn likely_terminal(state: &str) -> bool {
    // This is a naming heuristic, not a declaration of terminal-state semantics.
    let lower = state.to_lowercase();
    ["idle", "end", "done"].iter().any(|word| crate::utils::contains_keyword_bounded(&lower, word))
}

fn reachable(machine: &StateMachine) -> HashSet<String> {
    let mut reachable = HashSet::new();
    let mut pending = vec![machine.initial_state.clone()];
    while let Some(state) = pending.pop() {
        if !machine.states.contains(&state) || !reachable.insert(state.clone()) { continue; }
        pending.extend(machine.transitions.iter().filter(|t| t.from == state).map(|t| t.to.clone()));
    }
    reachable
}
