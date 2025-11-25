use crate::agents::orchestrator::Orchestrator;

pub struct AppState {
    pub orchestrator: Orchestrator,
}

impl AppState {
    pub fn new(orchestrator: Orchestrator) -> Self {
        Self { orchestrator }
    }
}