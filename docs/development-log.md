# Registro de desarrollo — Pomme Android

## 2026-10-30 — Fase 2.2-B

### Base inspeccionada

- Repositorio: `sylphie07/Pomme-Android-Test`
- Rama de trabajo: `phase-2.2-b-persistent-local-profile`
- Base: `c730ae9e1334646f507f9f068e8774e8950a18a8`
- El repositorio no tenía carpeta `docs/` en la rama de trabajo al comenzar esta actualización.

### Investigación

- Revisado `.github/workflows/ci.yml`: CI se ejecuta en push a `master`, pull request hacia `master` o ejecución manual.
- Consultada la lista de Actions de la rama: no hay ejecuciones registradas para ella.
- Revisados `Cargo.toml`, `pomme-client/Cargo.toml`, `rust-toolchain.toml` y recetas `ci-fmt`, `ci-lint`, `ci-test`, `ci-singleplayer` del `justfile`.
- El workspace usa Rust nightly fijado en `nightly-2026-08-21`.
- La receta de tests del cliente está en `just ci-test` y usa `cargo nextest run -p pomme-client --no-default-features`.

### Hallazgo y corrección

- Hallazgo: la prueba preexistente `local_profiles_create_persist_reload_and_select` trataba `LocalProfileStore::load_or_create` como un valor directo, aunque ahora devuelve `io::Result`.
- Cambio aplicado: se añadió `.unwrap()` en las dos llamadas de esa prueba.
- Commit: `c06d07e508772342707dd38c391ed47365a30bcb`.

### Estado de validación

- Revisión estática de código y workflow: realizada.
- Ejecución de compilación/tests/formato/clippy: no realizada.
- CI de GitHub: no ejecutado; no hay run en la rama.
- No afirmar compilación correcta ni tests aprobados.
- No se modificó `master`, no se abrió PR, no hubo merge ni APK.

### Pruebas adicionales añadidas

- Se amplió `legacy_local_profiles_migrate_with_the_historical_offline_uuid` para realizar un segundo arranque y comprobar que el UUID histórico no cambia.
- Añadida `missing_primary_profile_file_recovers_a_valid_backup` para comprobar la recuperación de una copia válida cuando falta el archivo principal.
- Commit: `071880da8042c028b9728d4db371150644840c9d`.
- Estas pruebas están añadidas, pero todavía no ejecutadas.
