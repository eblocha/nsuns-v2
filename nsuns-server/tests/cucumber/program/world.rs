use nsuns_server::program::model::{ProgramMeta, ProgramSummary};

#[derive(Debug, Default)]
pub struct ProgramWorld {
    pub program_meta: Option<ProgramMeta>,
    pub program_summary: Option<ProgramSummary>,
    pub programs_for_profile: Vec<ProgramMeta>,
}

impl ProgramWorld {
    pub fn unwrap_program_meta(&self) -> &ProgramMeta {
        self.program_meta
            .as_ref()
            .expect("No program metadata injected into global state")
    }

    pub fn unwrap_program_summary(&self) -> &ProgramSummary {
        self.program_summary
            .as_ref()
            .expect("No program summary injected into global state")
    }
}
