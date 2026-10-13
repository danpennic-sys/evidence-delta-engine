//! Evidence Delta Engine (EDE)
//!
//! Deterministic Learn-phase component of the Unified Adaptive Engine.
//! Consumes a BCAS and produces a Cycle Evolution Packet (CEP).

pub mod api;
pub mod applier;
pub mod auth;
pub mod collaborator;
pub mod compiler;
pub mod context;
pub mod cqrs;
pub mod error;
pub mod events;
pub mod formal;
pub mod governance;
pub mod models;
pub mod orchestrator;
pub mod saga;
pub mod scorer;
pub mod speculator;
pub mod state;
pub mod surfaces;
pub mod trace;
pub mod id;
pub mod timeutil;

pub use api::EvidenceDeltaEngine;
pub use applier::{Applier, ApplyResult, DefaultApplier, ApplyError};
pub use collaborator::DefaultCollaborator;
pub use compiler::{adaptive_cycle, compile, CompileResult};
pub use compiler::opt::OptPolicy;
pub use cqrs::{
    AdaptiveStateCommands, AdaptiveStateQueries, ApplyCepCommand, ApplyCepResult,
    CompensateCepCommand, CompensateCepResult, CqrsError, CurrentAdaptiveStateView, SagaQueries,
    SagaSummaryView, TimelineEntry,
};
pub use error::EdeError;
pub use auth::{AuthError, Authenticator, Credentials, InMemoryAuthenticator, Session, SessionToken};
pub use formal::{KernelAction, KernelTrace, LedgerValidator, Observation, PreservationCheck, TraceAdapter, TraceStep};
pub use governance::{DefaultGovernanceGate, GateContext, GateDecision, GateEvaluated, GovernanceGate, GovernancePolicy, RatificationRecord, RatificationService, RatifyCommand, Role, SealedGovernanceConfig};
pub use events::{
    streams, AggregateType, Event, EventPayload, EventStore, EventStoreError, EventType,
};
pub use models::*;
pub use id::Uuid;
pub use orchestrator::{
    CollaboratorPort, CycleOutcome, OrchestratorConfig, OrchestratorError, SagaOrchestrator,
    SpeculationInput, SpeculatorPort, StubCollaborator, StubSpeculator,
};
pub use saga::{
    AdaptiveCycleSaga, InMemorySagaStore, SagaState, SagaStore, SagaStoreError, StepId,
};
pub use speculator::{DefaultSpeculator, MHO};
pub use state::{
    optimistic_retry, AdaptiveState, AdaptiveStateStore, CompensateResult, Compensator, CreatedBy,
    DefaultCompensator, InMemoryStateStore, StateAdvancedEvent, StateError,
};
