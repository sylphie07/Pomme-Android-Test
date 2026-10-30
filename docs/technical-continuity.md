# Pomme Android — continuidad técnica

## Estado de la sesión

- Rama de trabajo: `phase-2.2-b-persistent-local-profile`
- Rama base de Fase 2.2-B: `unofficial-android-menu-access`
- HEAD base: `c730ae9e1334646f507f9f068e8774e8950a18a8`
- Último commit registrado al actualizar este documento: `071880da8042c028b9728d4db371150644840c9d`
- Estado: implementación de Fase 2.2-B en progreso; validación local/CI todavía no ejecutada.
- No se ha abierto PR, no se ha hecho merge, no se ha modificado `master`, no se ha generado APK.

## Objetivo de Fase 2.2-B

Añadir una identidad persistente de jugador independiente del identificador de registro de perfil, migrando los perfiles históricos sin cambiar sus UUID offline ni reemplazar archivos inválidos.

## Cambios aplicados

### `pomme-client/src/user.rs`

- `LocalProfile` ahora contiene `id`, `username` y `player_uuid`.
- Documento de perfiles versionado con `format_version = 1`.
- Lectura del formato antiguo: `player_uuid = offline_uuid(username)`.
- Validación de versión, perfiles vacíos, selección inexistente, IDs duplicados y UUID de jugador duplicados.
- Se rechazan documentos parcialmente versionados y UUID nulos.
- `load_or_create` devuelve `io::Result`; un archivo ilegible o inválido no se sustituye por un perfil nuevo.
- Escritura temporal única, sincronización del archivo, copia `.bak` y publicación por renombrado.
- Recuperación desde `.bak` cuando falta el archivo principal.
- Los perfiles recién creados mantienen por ahora el UUID offline del nombre para no cambiar el comportamiento real hasta que Fase 2.2-D conecte `player_uuid` al servidor integrado.
- Se actualizó la prueba existente para desenvolver el resultado fallible de `load_or_create`.

### `pomme-client/src/lib.rs`

- Si no se puede cargar el almacén de perfiles, el inicio se detiene con un error registrado; no se crea un perfil sustituto.

## Validaciones añadidas pero aún no ejecutadas

- Migración de perfil antiguo conservando `id`, selección y UUID offline histórico.
- Conservación de JSON corrupto.
- Rechazo de `format_version: null`.
- Rechazo de `player_uuid: null` en documentos antiguos.
- Rechazo de identidades históricas duplicadas.
- Persistencia del UUID tras recargar un documento versionado.
- Segundo arranque después de migrar un documento legacy, sin cambiar el UUID.
- Recuperación desde un backup válido cuando falta el archivo principal.
- Identidad offline inicial de perfiles nuevos.

## Limitaciones y riesgos abiertos

- No se ha ejecutado `cargo fmt`, `cargo test`, `cargo nextest`, `clippy` ni compilación Android.
- La conexión GitHub disponible permite inspección y escritura de archivos, pero no ofrece una acción de ejecución manual de Actions. El workflow CI del repositorio se dispara en push a `master`, PR hacia `master` o `workflow_dispatch`; la rama de trabajo no tiene ejecución registrada. No abrir PR ni tocar `master` sin autorización.
- No se ha demostrado todavía el comportamiento de `rename`/backup en Android ni en Windows.
- No hay bloqueo explícito para escrituras concurrentes; comprobar si el ciclo de vida de `LocalProfileStore` garantiza un único escritor antes de ampliar el diseño.
- La ruta actual de `UserData::from_local_profile` sigue derivando UUID offline del nombre. La utilización de `player_uuid` queda deliberadamente para Fase 2.2-D, después de implementar el canal confiable en SteelMC en Fase 2.2-C.
- Perfiles antiguos con nombres repetidos producen el mismo UUID histórico y se rechazan como conflicto; no se separan automáticamente.

## Próximos pasos obligatorios antes de Fase 2.2-C

1. Corregir cualquier error estático encontrado al revisar todas las llamadas a `load_or_create`.
2. Ejecutar formato, tests específicos de `pomme-client` y las recetas de CI si se dispone de un runner autorizado.
3. Confirmar migración real en una copia de perfil legacy y segundo arranque sin cambio de UUID.
4. Confirmar que el JSON corrupto permanece byte por byte sin modificaciones.
5. Probar recuperación desde backup y fallo durante publicación.
6. Revisar diff completo y estado de Actions.
7. Registrar resultados exactos y solo entonces declarar Fase 2.2-B validada.

## Restricciones

- No modificar `master`.
- No abrir PR ni hacer merge sin autorización explícita.
- No generar APK en esta fase.
- No comenzar Fase 2.2-C hasta validar B.
- Distinguir siempre cambios aplicados de pruebas ejecutadas y verificadas.
