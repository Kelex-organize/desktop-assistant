# Registro de decisiones

Cada decisión con su porqué. Si una cambia, no se borra: se agrega una nueva que la reemplaza y se marca la vieja como *reemplazada*.

Formato: **Contexto** (qué problema había) · **Decisión** · **Por qué** · **Alternativas consideradas**.

---

## 001 · Tauri 2 como base
- **Decisión:** Tauri 2 (Rust + WebView2).
- **Por qué:** liviana para una app siempre prendida; libertad de diseño con HTML/CSS; genera instalador; oportunidad de aprender Rust.
- **Alternativas:** Electron (más simple, pesado), C#/.NET (mejor integración con Windows, interfaz menos flexible), Python + Qt (mejor para IA y voz, peor como app distribuible).
- **Salida si falla:** Electron, conservando el frontend.

## 002 · React + TypeScript para la interfaz
- **Por qué:** varias ventanas y paneles; React ordena la interfaz y tiene más documentación. TypeScript por el tipado.

## 003 · Solo Windows
- **Por qué:** es mi sistema. Permite usar APIs propias de Windows sin abstraer.

## 004 · No es un chat
- **Decisión:** no hay interfaz de conversación. La IA solo interpreta pedidos y da confirmaciones cortas y amigables.
- **Por qué:** el objetivo es que me facilite tareas, no conversar.

## 005 · Módulos y registro de comandos
- **Decisión:** la app se organiza en módulos que exponen comandos a un registro central.
- **Por qué:** crecer de a poco sin tocar lo existente; un comando registrado queda disponible para la interfaz, el intérprete y la IA.

## 006 · Intérprete intercambiable, reglas primero
- **Decisión:** trait `Interpreter` con implementaciones por reglas, Claude y Ollama. Primero reglas.
- **Por qué:** permite tener voz antes que IA; la app funciona sin internet; las reglas quedan como respaldo; es fácil de testear.

## 007 · Toda la lógica en Rust
- **Decisión:** registro, intérprete, módulos, persistencia, audio y llamadas a la IA en Rust. React solo interfaz.
- **Por qué:** alarmas y voz funcionan con ventanas ocultas; la clave de API no pasa por el frontend; lógica testeable; más práctica de Rust.
- **Alternativa:** lógica de IA en TypeScript con el SDK oficial (más fácil, pero expone la clave al frontend).

## 008 · Contexto preparado, memoria después
- **Decisión:** el intérprete recibe un `Context` desde el inicio, vacío hasta el hito M7.
- **Por qué:** recordar pedidos anteriores no es necesario ahora, pero agregarlo después no debe obligar a rediseñar.

## 009 · Alarmas propias
- **Decisión:** módulo de alarmas propio en vez de usar la app Reloj de Windows.
- **Por qué:** la app Reloj no ofrece una forma documentada de crear alarmas desde otro programa; automatizarla sería frágil.

## 010 · Spotify con teclas multimedia
- **Decisión:** play/pausa/siguiente/anterior simulando las teclas multimedia de Windows.
- **Por qué:** funciona sin API ni login. La API de Spotify queda como idea para pedir canciones.

## 011 · Todo local
- **Decisión:** los datos quedan en la PC (SQLite). Sin sincronización.
- **Por qué:** simplicidad y velocidad de desarrollo. Sincronizar queda como idea.

## 012 · Idioma
- **Decisión:** código, comentarios, commits, ramas, issues y PRs en inglés. Documentación de `docs/` en español.
- **Por qué:** estándar de la industria, mejor para trabajar con IAs y para un repo público; la documentación en español para pensar cómodo.

## 013 · Prácticas profesionales
- **Decisión:** GitHub Issues + Projects, GitHub Flow, Conventional Commits, PRs con revisión, CI, tests, SemVer. Detalle en `FLUJO_DE_TRABAJO.md`.
- **Por qué:** aprenderlas y asegurar calidad.

## 014 · Nunca texto libre a una shell
- **Por qué:** evitar inyección de comandos, sobre todo cuando los parámetros vienen de una IA.

## 015 · Claude primero, Ollama después
- **Decisión:** el intérprete con IA se hace primero con Claude (Haiku); Ollama en M6.
- **Pendiente (M5):** API con pago aparte vs Agent SDK con la suscripción de Claude.

## 016 · Nombre
- **Decisión:** el asistente se llama **Kai** (provisorio). El repo usa un nombre neutro (`desktop-assistant`) hasta decidir el definitivo.

## 017 · Licencia MIT
- **Decisión:** el código se publica bajo la licencia MIT, con copyright a nombre de Emanuel Laguna.
- **Por qué:** quiero que cualquiera pueda usar el código y modificarlo, sin más condición que conservar el aviso de copyright. MIT es la más simple y común para este tipo de proyecto.
- **Alternativas:** Apache-2.0 (agrega una cláusula explícita de patentes, innecesaria acá); GPL (obliga a que las versiones derivadas también sean abiertas, más restrictiva de lo que busco).

## 018 · Diseño del registro de comandos
- **Decisión:** cada comando guarda su función como `Box<dyn Fn(Value) -> Result<CommandOutput, CommandError> + Send + Sync>`; los parámetros se validan contra su JSON Schema con el crate `jsonschema` (sin features por defecto); los errores son un `enum` con `thiserror`.
- **Por qué:** los módulos van a necesitar compartir estado (base de datos, `AppHandle`), y un closure puede capturarlo, a diferencia de un puntero a función. El esquema es una única fuente de verdad: valida las llamadas ahora y se le enviará a la IA después. `thiserror` es lo idiomático para errores tipados en Rust.
- **Por qué sin features por defecto:** `jsonschema` trae por defecto un cliente HTTP y TLS que no hacen falta, alargan la compilación y permitirían que un esquema descargue referencias de internet. Todo debe funcionar sin red.
- **Alternativas:** puntero a función `fn(...)` (más simple, sin estado); validación a mano (sin dependencias, pero el esquema y la validación pueden desincronizarse); `impl Display` a mano para los errores.

## 019 · Apps abribles: lista cerrada, fijas ahora y propias después
- **Decisión:** `open_app { name }` busca el nombre en una lista cerrada y lanza el ejecutable directo con `std::process::Command`, sin shell. En M1 la lista es un `enum` (`KnownApp`) con tres apps del sistema: Calculadora, Bloc de notas y Explorador. Las apps y juegos propios del usuario (por ejemplo `lol`) se agregan en M2 (issue #21), guardadas en SQLite.
- **Regla de seguridad:** agregar entradas a la lista de apps propias es una acción **solo de la interfaz**, nunca un comando del registro. Así ni el texto escrito ni la IA pueden registrar un destino nuevo; solo pueden elegir un alias que el usuario ya cargó.
- **Por qué:** una lista cerrada evita inyección de comandos aunque el nombre venga de la IA (decisión 014). Dividir en dos etapas evita construir SQLite y la ventana principal antes de tiempo.
- **Por qué `resolve` es una sola función:** es el punto donde más adelante se enchufa la lista del usuario, sin reescribir el resto.
- **Alternativas:** una tabla `&[(&str, &str)]` (más compacta, pero el compilador no obliga a cubrir todos los casos); `tauri-plugin-opener` (sirve para abrir páginas, no para lanzar una app por nombre).
