//! Sealed governance configuration - production path cannot pass arbitrary Role/Policy.

use super::{GateDecision, GovernancePolicy, Role};
use crate::id::Uuid;
use serde::{Deserialize, Serialize};

/// Authenticated principal (Phase-1: opaque id + role bound at construction).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Principal {
    pub principal_id: String,
    pub role: Role,
}

/// Loaded once from environment / secrets manager - not per-call caller args in prod.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SealedGovernanceConfig {
    pub policy: GovernancePolicy,
    pub principal: Principal,
    /// When true, Orchestrator ignores any attempt to override policy/actor on Config.
    pub sealed: bool,
}

impl SealedGovernanceConfig {
    pub fn bootstrap_system() -> Self {
        Self {
            policy: GovernancePolicy::default(),
            principal: Principal {
                principal_id: "system:bootstrap".into(),
                role: Role::System,
            },
            sealed: true,
        }
    }

    pub fn for_operator(principal_id: impl Into<String>, role: Role, policy: GovernancePolicy) -> Self {
        Self {
            policy,
            principal: Principal {
                principal_id: principal_id.into(),
                role,
            },
            sealed: true,
        }
    }
}

/// Durable gate decision record (must exist before Apply in hardened path).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateEvaluated {
    pub decision_id: Uuid,
    pub cycle_id: Uuid,
    pub cep_id: Uuid,
    pub decision: String,
    pub reason: Option<String>,
    pub principal_id: String,
    pub role: Role,
    pub policy_version: String,
    pub policy_generation: u64,
    pub recorded_at: String,
}

impl GateEvaluated {
    pub fn from_decision(
        cycle_id: Uuid,
        cep_id: Uuid,
        decision: &GateDecision,
        sealed: &SealedGovernanceConfig,
    ) -> Self {
        let (label, reason) = match decision {
            GateDecision::AutoAccept => ("auto_accept".into(), None),
            GateDecision::AutoReject { reason } => ("auto_reject".into(), Some(reason.clone())),
            GateDecision::HumanReviewRequired { reason } => {
                ("human_review".into(), Some(reason.clone()))
            }
            GateDecision::DualControlRequired { reason } => {
                ("dual_control".into(), Some(reason.clone()))
            }
        };
        Self {
            decision_id: Uuid::new_v4(),
            cycle_id,
            cep_id,
            decision: label,
            reason,
            principal_id: sealed.principal.principal_id.clone(),
            role: sealed.principal.role,
            policy_version: sealed.policy.version.clone(),
            policy_generation: sealed.policy.generation,
            recorded_at: crate::timeutil::now_rfc3339(),
        }
    }

    pub fn is_accept(&self) -> bool {
        self.decision == "auto_accept" || self.decision == "ratified"
    }
}
