use rig::providers::{anthropic, gemini, openai};

// =========================================================================
// OpenAI
// =========================================================================

/// Crea un modelo de completado de OpenAI.
/// Requiere OPENAI_API_KEY.
/// Ejemplo: gpt-4o, gpt-4o-mini, gpt-5, gpt-5-mini, gpt-5-nano, gpt-5.1-instant, gpt-5.1-thinking
pub fn openai(model: &str) -> openai::CompletionModel {
    openai::Client::from_env().completion_model(model)
}

// =========================================================================
// Anthropic (Claude)
// =========================================================================

/// Crea un modelo de completado de Anthropic.
/// Requiere ANTHROPIC_API_KEY.
/// Ejemplo: claude-3-7-sonnet-20250219, claude-sonnet-4-5-20250929, claude-haiku-4-5-20251001, claude-opus-4-5-20251101, claude-opus-4-1-20250805
pub fn anthropic(model: &str) -> anthropic::completion::CompletionModel {
    anthropic::Client::from_env().completion_model(model)
}

// =========================================================================
// Google (Gemini)
// =========================================================================

/// Crea un modelo de completado de Google Gemini.
/// Requiere GEMINI_API_KEY.
/// Ejemplo: gemini-2.5-flash, gemini-2.5-pro, gemini-3-pro-preview
pub fn gemini(model: &str) -> gemini::completion::CompletionModel {
    gemini::Client::from_env().completion_model(model)
}
