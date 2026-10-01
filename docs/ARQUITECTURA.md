# Arquitectura

## Idea central

La app es un conjunto de **módulos** (apps, alarms, notes, tft, media…). Cada módulo expone **comandos**: acciones con parámetros tipados, por ejemplo `open_app { name }`, `create_alarm { time, label }` o `tft_items { champion }`.

Hay dos formas de disparar un comando:

1. **Directa**: un botón o panel de la interfaz invoca un comando concreto.
2. **Por texto**: la barra rápida o la voz producen un texto libre. El **intérprete** lo convierte en un comando y, después, el comando se ejecuta.

```
 Barra rápida ──┐
                ├── texto ──▶ Intérprete ──▶ Intent ──▶ Registro de comandos ──▶ Módulo (Rust) ──▶ Windows / SQLite
 Voz (STT) ─────┘                                              ▲
                                                               │
 Botones y paneles ────────── comando directo ─────────────────┘

 Resultado ──▶ confirmación corta (texto en pantalla, personaje, voz)
```

## El intérprete

Recibe texto y devuelve una **intención**: qué comando ejecutar con qué parámetros, más la respuesta corta que hay que mostrar. Si el pedido no es un comando (un saludo), devuelve solo la respuesta.

Se define como un **trait** (en Java sería una interfaz) con varias implementaciones intercambiables:

```rust
// Boceto de diseño, no código final
trait Interpreter {
    fn interpret(&self, text: &str, context: &Context) -> Result<Intent, InterpretError>;
}

struct Intent {
    commands: Vec<CommandCall>,  // cero, uno o varios comandos
    reply: Option<String>,       // "¡Listo, alarma a las 7!" o respuesta a un saludo
}
```

| Implementación | Cuándo | Qué hace |
|---|---|---|
| `RuleInterpreter` | Desde el hito 2 | Frases con formas fijas ("alarma 7:00", "tft caitlyn", "pausa"), saludos con respuestas variadas. Funciona sin internet. Siempre queda como respaldo. |
| `ClaudeInterpreter` | Hito 5 | Entiende cualquier forma de pedirlo. Usa *tool use*: los comandos del registro se envían como herramientas. |
| `OllamaInterpreter` | Hito 6 | Lo mismo con un modelo local. Posible uso híbrido: Ollama para lo simple, Claude para lo complejo. |

El parámetro `context` hoy va vacío. Existe para agregar después memoria de pedidos anteriores ("mejor a las 8") sin rediseñar.

## El registro de comandos

Lista central con cada comando: **nombre, descripción, parámetros (JSON Schema) y la función que lo ejecuta**. La usan:
- el intérprete por reglas, para validar;
- el intérprete con IA, para enviarle las herramientas disponibles;
- la interfaz, por ejemplo para autocompletar en la barra rápida.

Agregar un comando al registro lo hace disponible en toda la app, incluida la IA.

## Qué va en Rust y qué en TypeScript

**Toda la lógica vive en Rust**: registro, intérprete, módulos, persistencia, audio, llamadas a la IA (así la clave de API nunca pasa por el frontend). **React solo muestra** y llama a Rust con `invoke`; Rust avisa cambios al frontend con **eventos** (por ejemplo, "sonó una alarma").

Por qué: las alarmas y la voz tienen que funcionar con las ventanas ocultas, la lógica queda testeable con `cargo test`, y es donde más se aprende Rust.

## Ventanas

| Ventana | Qué es |
|---|---|
| `character` | Personaje animado: transparente, sin bordes, siempre encima, arrastrable, ocultable |
| `launcher` | Barra rápida: aparece con un atajo global, se oculta al perder el foco o con Esc |
| `main` | Ventana completa con paneles de módulos y configuración |

Además: icono en la bandeja del sistema con menú.

## Persistencia

- **SQLite** (`rusqlite`) en la carpeta de datos de la app (`%APPDATA%`): alarmas, notas.
- **Configuración**: archivo JSON o `tauri-plugin-store`.
- **Secretos** (clave de API): Administrador de credenciales de Windows (crate `keyring`), nunca en archivos.

## Estructura de carpetas (objetivo)

```
src/                          # React (solo interfaz)
  windows/
    character/
    launcher/
    main/
  modules/                    # paneles de cada módulo para la ventana main
    alarms/
    notes/
  lib/
    api.ts                    # funciones tipadas que envuelven invoke()
src-tauri/src/
  main.rs
  lib.rs                      # arma la app: ventanas, plugins, comandos
  core/
    registry.rs               # registro de comandos
    intent.rs                 # Intent, CommandCall, Context
    interpreter/
      mod.rs                  # trait Interpreter
      rules.rs
      claude.rs               # hito 5
  modules/
    mod.rs
    apps.rs
    alarms.rs
    notes.rs
    tft.rs
    media.rs
  voice/                      # hito 3
  db.rs
```

## Anatomía de un módulo

1. `src-tauri/src/modules/<modulo>.rs`: la lógica, con tests unitarios en el mismo archivo (`#[cfg(test)]`).
2. Registrar sus comandos en `core/registry.rs`, con descripción y parámetros.
3. Si el intérprete por reglas lo debe entender, agregar sus frases en `rules.rs`, con tests.
4. Opcional: panel en `src/modules/<modulo>/` para la ventana principal.

## Decisiones abiertas

- **Reconocimiento de voz** (hito 3): Whisper local con `whisper-rs` (gratis, sin internet, más setup) o un servicio en la nube.
- **Voz de respuesta** (hito 3): voces de Windows (SAPI, gratis) o voces neurales.
- **Plan de Claude vs API** (hito 5): API con Haiku (pago aparte, muy barato para esto) o Agent SDK con la suscripción (condiciones cambiantes, más pesado).
