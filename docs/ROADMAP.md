# Roadmap

Los hitos se cargan como **Milestones** en GitHub y cada ítem como **Issue** (ver `FLUJO_DE_TRABAJO.md`). Este archivo es la vista general y el backlog inicial para crearlos. Después, **la fuente de verdad son los issues**.

Cada hito termina con algo usable.

---

## M0 · Setup
**Resultado:** proyecto creado, repo privado con CI y tablero listos.
Aprendo: toolchain de Rust y Tauri, estructura de un proyecto, GitHub Actions, Issues y Projects.

- Instalar herramientas: C++ Build Tools ("Desktop development with C++"), Rust (`stable-msvc`), Node LTS, Git, GitHub CLI (`gh`)
- VS Code: rust-analyzer, Tauri, ESLint, Prettier
- Crear proyecto con `npm create tauri-app@latest` (TypeScript, npm, React)
- Recorrer el ejemplo `greet`: `#[tauri::command]`, `generate_handler!`, `invoke`
- Repo privado en GitHub, `.gitignore`, protección de `main`
- Linters y formato: clippy, rustfmt, ESLint, Prettier; scripts `lint`, `typecheck`, `test`; Vitest
- CI con GitHub Actions
- Labels, milestones, Project board y carga de los issues de este archivo
- `README.md` y `CHANGELOG.md` iniciales

## M1 · Esqueleto
**Resultado:** apretar un atajo, escribir "calculadora", Enter, y se abre.
Aprendo: comandos Tauri, `Result` y `?`, `match`, structs y enums, ventanas y plugins.

- Icono en la bandeja con menú (mostrar, salir)
- Atajo global que muestra/oculta la barra rápida (`tauri-plugin-global-shortcut`)
- Ventana `launcher`: sin bordes, centrada, se oculta con Esc o al perder el foco
- Registro de comandos (`core/registry.rs`), primera versión
- Módulo `apps`: `open_app` con lista cerrada de apps (nada de texto libre a la shell)
- Mostrar el resultado o el error en la barra

## M2 · Intérprete por reglas y primeros módulos
**Resultado:** uso Kai todos los días para alarmas, TFT, música y notas, sin IA.
Aprendo: traits, manejo de strings, tests y TDD, SQLite, hilos y async, eventos Rust → React.

- Trait `Interpreter`, tipos `Intent` y `Context`; `RuleInterpreter` con TDD
- Saludos con respuestas amigables y variadas ("Kai, buenos días")
- Módulo `tft`: `tft_items { champion }` → MetaTFT, con normalización del nombre
- Módulo `media`: play/pausa, siguiente, anterior (teclas multimedia; funciona con Spotify)
- Módulo `apps` mejorado: índice de las apps del menú Inicio (incluye Microsoft Store) con búsqueda aproximada
- Base de datos SQLite y migraciones
- Módulo `alarms`: crear, listar, borrar; tarea en segundo plano; sonido y notificación con las ventanas ocultas
- Módulo `notes`: crear, listar, buscar
- Ventana `main` con paneles de alarmas y notas
- Autoinicio con Windows (activable)

## M3 · Voz
**Resultado:** aprieto un atajo, digo "items de Jinx" y se abre la página.
Aprendo: captura de audio, procesamiento en hilos, integración de librerías nativas.

- Decidir motor de reconocimiento (Whisper local con `whisper-rs` vs nube)
- Push-to-talk con atajo global y detección de fin de frase
- Transcripción y paso del texto al intérprete
- Indicador visual de "escuchando"
- Respuestas habladas cortas

## M4 · Personaje
**Resultado:** Kai vive en una esquina de la pantalla y reacciona.
Aprendo: ventanas transparentes, animación con CSS/Canvas, estado compartido entre ventanas.

- Ventana `character`: transparente, siempre encima, arrastrable, recuerda posición
- Estados animados: reposo, escuchando, pensando, hablando, contento
- Mostrar/ocultar desde menú y atajo
- Clic abre la barra rápida; menú contextual

## M5 · IA (Claude)
**Resultado:** entiende pedidos escritos de cualquier forma, incluso varios juntos.
Aprendo: HTTP en Rust (`reqwest`), serde/JSON, API de Claude y tool use, manejo de errores de red.

- Decidir API vs plan de Claude
- Guardar la clave en el Administrador de credenciales de Windows; configurarla desde la app
- `ClaudeInterpreter` con tool use a partir del registro
- Respuestas amigables cortas y personalidad de Kai
- Respaldo automático al intérprete por reglas sin internet
- Confirmación en la interfaz para acciones delicadas

## M6 · Ollama
**Resultado:** puedo usar Kai sin gastar nada.
- `OllamaInterpreter`
- Elegir motor en la configuración; evaluar modo híbrido (Ollama para lo simple, Claude para lo complejo)

## M7 · Contexto
**Resultado:** "poneme una alarma a las 7" … "mejor a las 8" funciona.
- Llenar `Context` con los últimos pedidos y resultados
- Tareas de varios pasos

## M8 · Publicación
- Instalador (`npm run tauri build`), icono y nombre definitivos
- README completo con capturas
- Hacer público el repo; primera GitHub Release
- Auto-actualización (updater de Tauri)

---

## Ideas (issues con etiqueta `idea`, sin hito)

- Spotify por API: pedir canciones o playlists por nombre
- Overlay de TFT: mostrar los ítems encima del juego sin abrir el navegador
- Timers y pomodoro
- Historial del portapapeles
- Volumen y brillo
- Clima
- Captura de pantalla
- Buscador de archivos
- Energía: bloquear, suspender, apagar (con confirmación)
- Palabra de activación ("Kai…") sin tocar el teclado
- Sincronizar alarmas con otros dispositivos
- Despertar la PC para alarmas (Programador de tareas)
- Automatizar otras apps con un sidecar en C# (FlaUI)
