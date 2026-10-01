# Flujo de trabajo

Aunque trabajo solo, uso las prácticas de un equipo profesional para aprenderlas y para que la app sea sólida.

## Seguimiento: GitHub Issues + Projects

- **Issue** = una unidad de trabajo (funcionalidad, bug, tarea técnica, idea). Título en inglés, descripción en español o inglés.
- **Milestones** = los hitos de `ROADMAP.md`. Cada issue pertenece a uno.
- **Project** (tablero Kanban): columnas `Backlog` → `Todo` → `In progress` → `In review` → `Done`.
- **Etiquetas**: `feature`, `bug`, `chore`, `docs`, `test`, `idea`, `good first rust` (buenas para practicar), y una por módulo: `module:alarms`, `module:tft`, etc.
- Toda idea nueva va como issue con `idea`, aunque sea para dentro de meses.

Un issue está bien escrito cuando dice **qué** se quiere, **por qué**, y tiene **criterios de aceptación** (cómo sé que está terminado).

## Ramas (GitHub Flow)

- `main` siempre funciona. **Nunca se commitea directo a `main`** (protegida en GitHub).
- Una rama por issue, desde `main` actualizada:
  - `feat/12-alarm-module`
  - `fix/31-launcher-focus`
  - `chore/5-ci-setup`
  - `docs/8-architecture`
- Ramas cortas: mejor varios PRs chicos que uno enorme.

## Commits: Conventional Commits

Formato `tipo(ámbito opcional): descripción en inglés, en imperativo`:

```
feat(alarms): add recurring alarms
fix(launcher): hide window on focus loss
test(rules): cover time parsing edge cases
refactor(registry): extract command validation
chore(deps): update tauri to 2.x
docs: explain interpreter design
```

Commits chicos y con sentido propio. Se escriben en inglés.

## Pull Requests

1. Push de la rama y PR contra `main`.
2. Descripción: qué cambia, por qué, cómo probarlo, y `Closes #N` para cerrar el issue al mergear.
3. **Revisión**: pedirle a Claude Code que revise el PR como lo haría un compañero senior.
4. **CI en verde** obligatorio.
5. **Squash merge** (un commit limpio por PR en `main`) y borrar la rama.

## Tests

- **Rust**: tests unitarios en cada archivo (`#[cfg(test)] mod tests`), con `cargo test`. Todo lo que es lógica pura (intérprete por reglas, parseo de horas, normalización de nombres, validaciones) **tiene tests**.
- **TypeScript**: Vitest para la lógica de la interfaz que lo justifique.
- Un bug corregido lleva un test que lo reproduce.
- El intérprete por reglas es ideal para practicar **TDD**: escribir primero el test ("'alarma 7' → create_alarm 07:00") y después el código.

## Calidad automática (CI)

GitHub Actions corre en cada PR, sobre Windows:
- `cargo fmt --check` (formato)
- `cargo clippy -- -D warnings` (linter de Rust; enseña mucho Rust idiomático)
- `cargo test`
- `npm run lint`, `npm run typecheck`, `npm test`
- Build de Tauri

Localmente, lo mismo antes de hacer push.

## Versiones y cambios

- **Versionado semántico**: `0.x.y` mientras está en desarrollo. Cada hito terminado = versión minor (`0.1.0`, `0.2.0`…).
- **CHANGELOG.md** actualizado en cada versión.
- **Tags** y **GitHub Releases** con el instalador adjunto.

## Secretos y repo público

- El repo arranca **privado** y se hace público más adelante.
- Desde el primer commit: nada de claves, tokens ni datos personales en el código ni en el historial.
- `.gitignore` desde el inicio: `target/`, `node_modules/`, `dist/`, `.env*`, bases de datos locales.
- La clave de API se guarda en el Administrador de credenciales de Windows y se carga desde la configuración de la app.

## Definición de terminado

Un issue está terminado cuando:
- [ ] Cumple los criterios de aceptación
- [ ] Tiene tests de la lógica nueva
- [ ] CI en verde
- [ ] Revisado y mergeado a `main`
- [ ] Documentación actualizada si cambió algo de diseño (`docs/`)
