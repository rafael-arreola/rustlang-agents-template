# Prompt de Desarrollo y Arquitectura - Rust AI Agent Template

Este documento sirve como contexto maestro para el desarrollo de sistemas basados en Agentes de IA en este proyecto. Define la arquitectura, el flujo de datos y las reglas para la implementación de nuevos agentes y herramientas.

## 1. Arquitectura Agéntica

El sistema sigue un patrón de **Orquestación y Especialización**. A diferencia de un MVC tradicional, aquí la lógica de negocio reside en la interacción entre modelos de lenguaje (LLMs) y herramientas ejecutables.

### Mapa del Territorio

- **`src/agents`**: **El Cerebro.** Contiene la lógica de los agentes.
  - **`orchestrator`**: El agente principal que recibe la intención del usuario y decide qué especialista activar.
  - **`specialized`**: Agentes expertos en una tarea única (ej. `address`, `damage`). Son invocados como herramientas.
  - _Regla_: Los `system_prompt.md` deben vivir junto al código del agente para mantener contexto y lógica unidos.

- **`src/tools`**: **Las Manos.** Funciones deterministas que los agentes pueden ejecutar.
  - Aquí se definen las estructuras que implementan el trait `Tool` de la librería `rig`.
  - _Regla_: Las herramientas deben ser puras o manejar sus propios side-effects de forma aislada.

- **`src/infra`**: **El Sistema Nervioso.** Conexiones a servicios externos.
  - **`llm.rs`**: Configuración centralizada de proveedores (OpenAI, Anthropic, Gemini).
  - **`telemetry.rs`**: Observabilidad.
  - _Regla_: Nunca instancies un cliente de LLM (`Client::from_env`) fuera de esta capa.

- **`src/api`**: **Los Sentidos.** La interfaz HTTP.
  - Recibe peticiones externas y se las pasa al `Orchestrator`. No contiene lógica de negocio, solo transformación de DTOs.

- **`src/state.rs`**: **Memoria a Corto Plazo.**
  - Mantiene el estado compartido de la aplicación (referencias a los agentes inicializados).

## 2. Stack Tecnológico & Estándares

- **Framework de Agentes**: `rig` (Rust Intelligent Graph).
- **Inyección de Modelos**: Se prefiere la inyección explícita de modelos en los constructores de los agentes para facilitar el testing y el cambio de proveedores (ej. cambiar GPT-4 por Claude 3.5 Sonnet según la tarea).

### Reglas de Implementación

1.  **Prompts como Código**: Los System Prompts deben estar en archivos `.md` separados (ej. `system_prompt.md`) y cargados con `include_str!`. Esto facilita la lectura y edición sin recompilar cadenas gigantes en Rust.
2.  **Tipado Fuerte**: Usa structs de Rust para definir los inputs y outputs de las Tools. Aprovecha el sistema de tipos para validar antes de que el LLM "alucine".
3.  **Modularidad**: Un agente no debe saber de la existencia de la API HTTP. La API importa al agente, nunca al revés.

## 3. Flujo de Trabajo para Nuevas Features

Para añadir una nueva capacidad al sistema:

1.  **Definir la Tool (Opcional)**: Si el agente necesita hacer algo físico (buscar en DB, calcular envío), crea la herramienta en `src/tools`.
2.  **Crear el Especialista**:
    - Crea `src/agents/specialized/NUEVO_AGENTE.rs`.
    - Define su `system_prompt.md`.
    - Configura qué modelo usará (generalmente uno más rápido/barato si la tarea es simple).
3.  **Registrar en el Orquestador**:
    - Añade el nuevo especialista como una `.tool()` en `src/agents/orchestrator/mod.rs`.
    - Actualiza el prompt del orquestador si es necesario para que sepa cuándo usarlo.
4.  **Exponer (Si aplica)**: Si el agente requiere un endpoint directo (raro, usualmente es vía orquestador), añádelo en `src/api`.

---

**Nota para IA:** Al generar código, recuerda que estamos usando `rig`. Verifica `src/infra/llm.rs` para ver qué modelos están disponibles y pre-configurados. Prioriza el uso de `Gemini Flash` para tareas de alta velocidad y `Claude Sonnet` o `GPT-4o` para razonamiento complejo.
