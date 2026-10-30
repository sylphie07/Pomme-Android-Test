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

### Robustez multiplataforma de publicación

- Hallazgo de revisión: `std::fs::rename` no reemplaza un destino existente en Windows, lo que habría hecho fallar escrituras posteriores cuando ya existía `.bak` o el documento principal.
- Cambio aplicado: helper `publish_file`; POSIX conserva reemplazo por rename; Windows mueve temporalmente el destino previo, publica el nuevo archivo y restaura el anterior si falla el reemplazo. La copia `.bak` continúa siendo el mecanismo de recuperación ante interrupciones.
- Commit: `66bd785866440142398b3d6ae442421c10040535`.
- No se ejecutaron pruebas Windows ni Android.

### Identidad para perfiles nuevos

- Ajuste de diseño: la arquitectura aprobada requiere UUID v4 independiente para perfiles nuevos; solo los perfiles legacy reciben el UUID offline histórico derivado del nombre.
- Se actualizó `LocalProfile::new` y se añadió regeneración si un UUID nuevo colisiona con uno ya almacenado.
- La ruta de juego todavía usa UUID offline hasta Fase 2.2-D; esta rama intermedia no debe distribuirse como versión final antes de conectar y validar la identidad persistente.
- Se actualizó la prueba correspondiente.
- Commit: `e247812983cc1879efdc4cb1f8db00fb1011e841`.

### Intento de validación en el entorno disponible

- Revisadas todas las referencias visibles a `LocalProfileStore::load_or_create`: el inicio en `pomme-client/src/lib.rs` maneja el `Result` y las pruebas usan `.unwrap()` o `.is_err()` según el caso.
- Se intentó comprobar herramientas locales: no están instalados `rustc`, `cargo` ni `rustfmt`.
- Se intentó clonar la rama para ejecutar búsquedas locales; el entorno no pudo resolver `github.com` (DNS), así que no se obtuvo una copia local.
- Consultada Actions: cero ejecuciones asociadas a la rama.
- No se ejecutó ninguna prueba ni compilación. No se declara validación aprobada.
