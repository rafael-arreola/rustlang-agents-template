use super::specialized::{address::AddressSpecialist, damage::DamageSpecialist};
use crate::infra::llm;
use rig::{
    agent::{Agent, AgentBuilder},
    providers::openai,
};

pub struct Orchestrator {
    pub agent: Agent<openai::CompletionModel>,
}

impl Orchestrator {
    pub fn new() -> Self {
        // 1. Configuramos los Modelos (Strings manuales con sintaxis limpia)

        let gpt_model = llm::openai("gpt-4o");
        let gemini_model = llm::gemini("gemini-1.5-flash-002");
        let sonnet_model = llm::anthropic("claude-3-5-sonnet-20241022");

        // 2. Inicializamos los Sub-Agentes (Tools)
        let address_tool = AddressSpecialist::new(gemini_model);
        let damage_tool = DamageSpecialist::new(sonnet_model);

        // 3. Construimos el Agente Principal
        let agent = AgentBuilder::new(gpt_model)
            .preamble(include_str!("system_prompt.md"))
            .tool(address_tool)
            .tool(damage_tool)
            .build();

        Self { agent }
    }
}
