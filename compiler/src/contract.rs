// Contract Checker — @contract state machine simulation + early violation detection
use super::ast::*;
use std::collections::{HashMap, HashSet};
pub(crate) mod definition;
mod integrity;

/// State machine definition
#[derive(Debug)]
pub struct StateMachine {
    pub name: String,
    pub states: Vec<String>,
    pub initial_state: String,
    pub transitions: Vec<Transition>,
    pub invariants: Vec<String>,
}

#[derive(Debug)]
pub struct Transition {
    pub from: String,
    pub to: String,
    pub on_event: String,
}

/// State machine runtime (for simulation)
#[derive(Debug)]
pub struct ContractRuntime {
    pub machines: HashMap<String, StateMachine>,
    pub current_states: HashMap<String, String>,
    definition_issues: Vec<ContractViolation>,
}

/// Contract check result
#[derive(Debug, Clone)]
pub struct ContractViolation {
    pub contract: String,
    pub violation_type: ViolationType,
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum ViolationType {
    InvalidTransition,
    MissingState,
    UnreachableState,
    DeadlockDetected,
}

impl std::fmt::Display for ContractViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let icon = match self.violation_type {
            ViolationType::InvalidTransition => "🚫",
            ViolationType::MissingState => "❓",
            ViolationType::UnreachableState => "🔒",
            ViolationType::DeadlockDetected => "💀",
        };
        write!(f, "{} [{}] {:?}: {}", icon, self.contract, self.violation_type, self.message)
    }
}

impl ContractRuntime {
    /// Extract @contract blocks from N2File and build state machines
    pub fn from_file(file: &N2File) -> Self {
        let mut machines = HashMap::new();
        let mut definition_issues = Vec::new();
        for block in &file.blocks {
            let Block::Contract(contract) = block else { continue; };
            definition_issues.extend(definition::simulation_issues(file, contract));
            if contract.states.is_none() { continue; }
            let states = definition::states(file, contract).unwrap_or_default();
            let initial_state = contract.transitions.first().map(|t| t.from.clone())
                .or_else(|| states.first().cloned()).unwrap_or_default();
            machines.entry(contract.name.clone()).or_insert_with(|| StateMachine {
                name: contract.name.clone(), states, initial_state,
                transitions: contract.transitions.iter().map(|t| Transition {
                    from: t.from.clone(), to: t.to.clone(), on_event: t.on_event.clone(),
                }).collect(), invariants: contract.invariants.clone(),
            });
        }
        let current_states = machines.iter().map(|(name, machine)| (name.clone(), machine.initial_state.clone())).collect();
        Self { machines, current_states, definition_issues }
    }

    /// Static analysis includes declaration errors and advisory graph checks.
    pub fn check_integrity(&self) -> Vec<ContractViolation> {
        let mut issues = self.definition_issues.clone();
        for machine in self.machines.values() { issues.extend(integrity::check(machine)); }
        issues.sort_by(|a, b| (&a.contract, &a.message).cmp(&(&b.contract, &b.message)));
        issues
    }

    /// Event simulation: check whether a transition is valid
    pub fn simulate_event(&mut self, contract_name: &str, event: &str) -> Result<String, ContractViolation> {
        if let Some(issue) = self.definition_issues.iter().find(|issue| issue.contract == contract_name) {
            return Err(issue.clone());
        }
        let current = self.current_states.get(contract_name)
            .ok_or_else(|| ContractViolation {
                contract: contract_name.to_string(),
                violation_type: ViolationType::MissingState,
                message: format!("계약 '{}'를 찾을 수 없습니다", contract_name),
            })?
            .clone();

        let machine = self.machines.get(contract_name)
            .ok_or_else(|| ContractViolation {
                contract: contract_name.to_string(),
                violation_type: ViolationType::MissingState,
                message: format!("상태머신 '{}'을 찾을 수 없습니다", contract_name),
            })?;

        // Find a valid transition
        let valid_transition = machine.transitions.iter()
            .find(|t| t.from == current && t.on_event == event);

        match valid_transition {
            Some(t) if machine.states.contains(&t.to) => {
                let new_state = t.to.clone();
                self.current_states.insert(contract_name.to_string(), new_state.clone());
                Ok(new_state)
            }
            _ => {
                let valid_events: Vec<_> = machine.transitions.iter()
                    .filter(|t| t.from == current)
                    .map(|t| t.on_event.as_str())
                    .collect();
                Err(ContractViolation {
                    contract: contract_name.to_string(),
                    violation_type: ViolationType::InvalidTransition,
                    message: format!(
                        "상태 '{}'에서 이벤트 '{}'는 유효하지 않습니다. 가능한 이벤트: [{}]",
                        current, event, valid_events.join(", ")
                    ),
                })
            }
        }
    }

    /// Print a summary of state machines
    pub fn summary(&self) -> String {
        let mut lines = Vec::new();
        let mut machines: Vec<_> = self.machines.iter().collect();
        machines.sort_by_key(|(name, _)| *name);
        for (name, machine) in machines {
            let current = self.current_states.get(name).unwrap_or(&"?".to_string()).clone();
            lines.push(format!("📋 {} | states: {} | current: {} | transitions: {}",
                name,
                machine.states.len(),
                current,
                machine.transitions.len(),
            ));
            for t in &machine.transitions {
                lines.push(format!("   {} -[{}]-> {}", t.from, t.on_event, t.to));
            }
        }
        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_n2;

    #[test]
    fn test_state_machine_simulation() {
        let source = r#"
@meta {
  name: "test-contract"
  enforce: strict
}

@contract SessionLifecycle {
  scope: session
  states: SessionState

  transitions {
    IDLE -> BOOTING : on n2_boot
    BOOTING -> READY : on boot_complete
    READY -> WORKING : on n2_work_start
    WORKING -> IDLE : on n2_work_end
  }
}
"#;
        let file = parse_n2(source).unwrap();
        let mut runtime = ContractRuntime::from_file(&file);

        // Verify machines exist
        assert!(!runtime.machines.is_empty(), "state machines are empty");
        assert!(runtime.machines.contains_key("SessionLifecycle"), "SessionLifecycle machine not found");

        // Initial state: IDLE
        let initial = runtime.current_states.get("SessionLifecycle").cloned();
        assert_eq!(initial, Some("IDLE".to_string()), "initial state should be IDLE");

        // IDLE -> BOOTING (n2_boot)
        let result = runtime.simulate_event("SessionLifecycle", "n2_boot");
        assert!(result.is_ok(), "transition failed: {:?}", result.err());
        assert_eq!(result.unwrap(), "BOOTING");

        // BOOTING -> READY (boot_complete)
        let result = runtime.simulate_event("SessionLifecycle", "boot_complete");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "READY");

        // n2_boot is not valid from READY
        let result = runtime.simulate_event("SessionLifecycle", "n2_boot");
        assert!(result.is_err());
    }

    #[test]
    fn test_integrity_check() {
        let source = r#"
@meta {
  name: "test"
  enforce: strict
}

@contract Test {
  states: S

  transitions {
    A -> B : on go
    B -> C : on next
    C -> A : on reset
  }
}
"#;
        let file = parse_n2(source).unwrap();
        let runtime = ContractRuntime::from_file(&file);
        let violations = runtime.check_integrity();
        // A->B->C->A cycle: no unreachable states or deadlocks expected
        assert!(violations.is_empty(), "violations detected: {:?}", violations);
    }
}
