use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrchestratorError {
    #[error("EDE error: {0}")]
    Ede(String),

    #[error("Apply error: {0}")]
    Apply(String),

    #[error("Apply blocked: {0}")]
    ApplyBlocked(String),

    #[error("Compensate error: {0}")]
    Compensate(String),

    #[error("Saga store error: {0}")]
    SagaStore(String),

    #[error("CEP rejected: {0}")]
    CepRejected(String),

    #[error("Gate rejected: {0}")]
    GateRejected(String),

    #[error("KernelTrace validation failed: {0}")]
    TraceValidation(String),

    #[error("Internal orchestrator error: {0}")]
    Internal(String),
}
