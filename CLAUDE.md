# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

BrickRust is an SDK and mod loader for **Brick Rigs** (Steam ID: 552100), written in unsafe Rust. It maps Unreal Engine 4 structures, hooks engine functions, and extends game functionality via brick property customization and blueprint interception.

The project is a cargo workspace with five crates plus one proxy crate:

| Crate | Type | Purpose |
|-------|------|---------|
| **brickrust** | dylib | Main SDK — UE4 structure mapping, function hooks, mod APIs |
| **brickworks** | rlib | Shared interface definitions, signature lookup macros, mod loading logic |
| **brickworks_impl** | cdylib | The actual `brickworks.dll` — implements brickworks interface via platform-specific backends |
| **brickrust_macros** | proc-macro | The `sig!()` macro for compile-time byte-pattern signatures |
| **brmk_plugin** | cdylib | BrickRigs Mod Kit Steam plugin variant of brickworks |
| **xinput_proxy** | cdylib | Shims `xinput1_3.dll` and loads `brickworks.dll` at startup |

## Building and Installing

```sh
# Release build (install to Brick Rigs folder)
make install DIR="/path/to/common/Brick Rigs"

# Development build
make install DIR="/path/to/common/Brick Rigs" dev=true

# BRMK variant build
make install_brmk DIR="/path/to/common/Brick Rigs"

# Generate docs (targets windows)
make doc
```

The install step copies `xinput1_3.dll`, `brickworks.dll`, mingw runtime DLLs (`libgcc_s_seh-1.dll`, `libwinpthread-1.dll`), and example mods into the Brick Rigs installation directory.

For Proton/Wine compatibility: add `WINEDLLOVERRIDES="xinput1_3=n,b" %command%` to Steam launch options.

## Running Tests

```sh
cargo test --target x86_64-unknown-linux-gnu --workspace --exclude brickrust
```

Tests exist in the `brickworks::patterns` module (signature lookup tests). The main brickrust crate does not have unit tests — it is runtime hooking code for Windows.

## Workspace Features

| Feature | Effect |
|---------|--------|
| `brmk` | Enables BRMK (BrickRigs Mod Kit Steam) plugin support; enables `brickworks/brmk` |
| `brmk_1_10_7` | Maps BRMK symbols for version 1.10.7 |
| `br_1_11` | Maps Brick Rigs version 1.11 symbols |
| `br_1_10_7` | Maps Brick Rigs version 1.10.7 symbols |
| `brickworks_impl/impl` | Enables the actual implementation (win32/win_universal) in brickworks_impl; without this, brickworks_impl is the stub |

## Architecture

### Crate relationship

```
brickworks_impl (cdylib, builds as brickworks.dll)
  └── uses: brickworks (rlib) for interface definitions
  └── uses: brickrust_macros for sig! macro
  └── backends: win32.rs (Windows DLL) | brmk.rs (BRMK plugin) | stub.rs (fallback)

brickrust (dylib, the SDK for mod authors)
  └── depends on: brickworks (rlib) for lookup!, sig!, br_print!
  └── depends on: brickrust_macros (proc-macro) for sig!
  └── depends on: min_hook_rs, inventory, backtrace

xinput_proxy (cdylib, builds as xinput1_3.dll)
  └── loads brickworks.dll at DLL_PROCESS_ATTACH
```

### Entry points

Mods must export two symbols (no mangling):
- `mod_info()` → returns a `ModInfo` struct with name, description, version, game_version, authors
- `mod_init()` → called before engine init; must call `brickrust::init()` and set up hooks

The mod loader (`brickworks.dll`) scans `brickworks/` and `BrickRigs/Mods/*/` for `.dll` files, loads each via `libloading`, resolves `mod_info` and `mod_init`, and calls each mod's `mod_init()`.

Mod filenames prefixed with `_` are marked as disabled (e.g., `_disabled.dll` → skipped).

### Hooking system

The hooking pipeline uses **min_hook_rs** (ETW hooks) for inline function hooking. `brickworks_impl` provides the `brickworks_hook_internal()` binding that actually installs hooks. The `hookmgr.rs` module maintains a HashMap of hooked functions and their pre/post hook callbacks.

Key hooks (non-BRMK):

1. **UEngine::Init** — `hook_post_engine_init()` fires after engine init
2. **UEngine::LoadMap** — `hook_post_load_map()` fires when a map loads
3. **StaticConstructObject_Internal** — `hook_construct_uobject()` intercepts every UE4 object construction

Subhook callbacks receive either `*mut UObjectBase` (object init) or raw function pointers (post-init/map callbacks).

#### Pre/post hook model

Each hooked function can have:
- **prehooks** — called before the original function
- **posthooks** — called after the original function

Access via `brickworks_add_prehook()`/`brickworks_get_prehooks()` and `brickworks_add_posthook()`/`brickworks_get_posthooks()`.

### Signature scanning

Functions are located by byte-pattern signatures at runtime via the `lookup!` macro, which registers `InventoryLookupInfo` entries. During `do_lookup()` (called in `init_signatures()`), each entry is resolved through the backend's `brickworks_binary_lookup()`, `brickworks_binary_dll_lookup()`, or `brickworks_cpp_lookup()` function.

The `sig!()` macro (from brickrust_macros) compiles byte patterns with `?`/`??` wildcards into `Signature` structs containing byte arrays and boolean masks.

Lookup modes:
- `SignatureStart` — use offset directly
- `Offset32` — read a 32-bit relative offset from the matched address
- `Direct64` — read a 64-bit absolute address

Lookup info types:
- `Binary` — scan the game binary directly
- `BinaryDll` — scan a specific DLL (used in BRMK)
- `Proc` — resolve via `GetProcAddress` by decorated name
- `ProcMangled` — resolve via `GetProcAddress` by C++ mangled name

### UE4 structure mapping (`src/ue/`)

Maps core Unreal Engine 4 types:
- `coreuobject.rs` — `UObject`, `UObjectBase`, `FUObjectArray`, vtables, object iteration
- `uclass.rs` — `UClass`, `UStruct`, `UField`, `FField`
- `fname.rs` — `FName` (UE4 name system)
- `fproperty.rs` — `FProperty` and vtables
- `fstring.rs` / `ftext.rs` — string/UScriptName types
- `fframe.rs` — `FFrame` (UE4 execution stack frames)
- `tarray.rs` / `tmap.rs` / `tpair.rs` / `tshared.rs` / `toptional.rs` — UE4 containers
- `fmalloc.rs` / `fassetregistry.rs` / `farchive.rs` — memory and serialization
- `blueprint.rs` — Blueprint function interception via `inventory`-driven dispatch
- `gameplay/` — `AActor`, `UWorld`, `FActorSpawnParameters`, `GWorld`, `SpawnActor`

### Game-specific structures (`src/br/`)

All types follow [Redacted00/BrickRigs-Headers]. Leaf modules:
- `assetmgr.rs` — `ELoadAssetLibrariesMode`, `UBrickAssetManager`
- `statics.rs` — `GetProjectVersion()`, `IsModdedAsset()`
- `utils.rs` — utility functions
- `modhook.rs` — mod hook infrastructure

Subdirectory modules:
- `brickeditor/` — `UBrickEditorObjectStaticInfo`, `UBrickEditorObjectStaticInfoVTable`, `FBrickRigsSaveVersion`
- `bricks/` — `UBrick` (+ `UBrickFlags1/2/3`), `UScalableBrick`, `UMotorBrick` (`UMotorBrickFlags`), `UActuatorBrick` (`FActuatorState`, `EActuatorMode`, `UActuatorBrickFlags`), `UThursterBrick`, `FFuelTankParams`, `FBrickTickFunction`, `FBrickDamage`
- `vehicle/` — `ABrickVehicle` (`ABrickVehicleFlags1/2`, `FViewTargetZoomCache`), `UBrickConnection` (`FBrickConnectionParams`, `UBrickConnectionFlags`, `EConnectorType`), `UPhysicsConstraintConnection`, `FRepVehicleMovement`, `FVehicleInputChannel`, `EVehicleInputAxis`
- `game/` — `UBrickGameInstance`, `ISpawnPointInterface`
- `properties/` — `IBrickPropertyInterface` (+ vtable), `FBrickProperty` (+ vtable, instance), `FNumericBrickPropertyBase` (+ vtable, value, range), `FBrickPropertyEditInfo`, `FBrickPropertyReflection` (+ filter), `FTextBrickProperty`, `TBrickPropAttribute<T>`, `FBrickPropertyContainer`, `ENumericValueType`, `EFluAxisLock`
- `items/` — `AInventoryItem`
- `projectiles/` — `EAmmoType`


### VTable and memory manipulation (`src/utils/`, `src/really_scary.rs`)

- `utils/vtable.rs` — copies vtables to writable memory so mods can override methods
- `really_scary.rs` — `uclass_reserve_memory()` / `uclass_reserve_memory2::<T>()` expand `UClass` allocation size to embed custom struct fields inline

### Mod dispatch

The `inventory` crate stores `BlueprintFunction` structs; during `ProcessEvent` interception, the engine dispatches to registered blueprint functions by matching function name and class.

Use the `bp_function!()` macro to register:
```rust
bp_function(my_function_name | obj, stack, result | { /* body */ });
bp_function(Some("MyClass"), "MyFunction", |obj, stack, result| { /* body */ });
```

### Field naming conventions (`src/ue/`)

- **VTable structs** (e.g., `UObjectVTable`, `AActorVTable`) — field names use **PascalCase**, matching UE4's C++ vtable method names
- **Data structs** (e.g., `UObjectBase`, `AActor`) — field names use **snake_case**

### Field naming conventions (`src/br/`)

All variable names follow [Redacted00/BrickRigs-Headers]. For fields with unknown purposes: `_aX` where `X` is an incrementing index (`_a1`, `_a2`, ...).

Bitflag variants use **SCREAMING_SNAKE_CASE** without the `b` prefix. Bitflag struct sizes must match the C++ layout: `u8` for `uint8`, `u16` for `uint16`, etc.

## DLL / Engine / Map lifecycle

The mod loader and SDK follow a strict initialization pipeline.

### Stage 1: DLL load (`brickworks.dll`)

`brickworks_init()` runs. This:
1. Initializes the hook manager (`min_hook_rs`)
2. Scans `brickworks/` and `BrickRigs/Mods/*/` for `.dll` files
3. Loads each mod DLL via `libloading`
4. Resolves `mod_info` from each mod and prints metadata
5. Calls each mod's `mod_init()`

**No engine functions are available.** This stage is for setting up vtable hooks and early signatures.

### Stage 2: Engine init (`UEngine::Init`)

After the game's own `UEngine::Init` completes, the SDK fires all callbacks registered via `hook_post_engine_init()`. `init_signatures()` resolves all UE4 and game function pointers. Blueprint functions are ready.

Register with:
```rust
brickrust::hook_post_engine_init(my_engine_init);
```

The SDK's own `engine_load()` callback (registered internally) scans `GObjects()` and sets `UCLASS` to the "Class" UClass.

### Stage 3: Map load (`UEngine::LoadMap`)

After the game's map load completes, all callbacks registered via `hook_post_load_map()` fire. `GObjects()` contains the full object graph for the current level; `GNames()` has the full name table.

Register with:
```rust
brickrust::hook_post_load_map(my_map_init);
```

### Stage 4: Object construction (ongoing)

`StaticConstructObject_Internal` is hooked for the process lifetime. Every UE4 object creation dispatches to all subhooks registered via `hook_construct_uobject()`. Use this to:
- Replace vtables on specific class types
- Inject custom data into new instances
- Intercept CDO (Class Default Object) creation

Register with:
```rust
brickrust::hook_construct_uobject(my_object_init);
```

### Summary table

| Stage | Trigger | APIs available | Registration |
|-------|---------|----------------|--------------|
| DLL load | Game loads brickworks.dll | None (pre-engine) | — |
| Engine init | After `UEngine::Init` | Version checks, blueprint checks | `hook_post_engine_init()` |
| Map load | After `UEngine::LoadMap` | Full object traversal, GObjects, GNames | `hook_post_load_map()` |
| Object construction | Every `StaticConstructObject_Internal` | Vtable manipulation, instance data | `hook_construct_uobject()` |

## Key development patterns

- All engine/game function pointers are **resolved at runtime**, not linked at compile time
- All code interacting with the game uses `unsafe` — raw pointers, vtable manipulation, transmute
- Structs are `#[repr(C)]` and manually aligned to match the game's memory layout
- Use `container_of!` macro from brickrust_macros for pointer arithmetic
- Module names are set via `set_module_name!()` for log routing

## Important constraints

- **Target only**: `x86_64-pc-windows-gnu`
- **Panics on abort**: Both dev and release profiles use `panic = "abort"` — no unwinding
- **Runtime dependencies**: The game binary needs `xinput1_3.dll`, `brickworks.dll`, and mingw runtime DLLs in the right paths
- **Version checks**: Mods should run `warn_version_mismatch!()` or `panic_version_mismatch!()` after engine init (not in `mod_init`)
- **Blueprint mod checks**: Use `check_blueprint_mod()` / `ensure_blueprint_mod()` to validate required mods

## Examples

Five example mods are included in the `examples/` directory:

| Example | Description |
|---------|-------------|
| `basic_init.rs` | Basic mod setup with vtable replacement on UBrick |
| `custom_brick_properties.rs` | Custom brick properties via `uclass_reserve_memory` + property reflection |
| `function_tests.rs` | Static game function usage (`GetProjectVersion`, `GetEnabledModNames`) |
| `better_crashes.rs` | Simpler engine init hook |
| `workshop_allow_modded.rs` | Workshop modded asset handling |

All examples follow the same pattern: `mod_info()` + `mod_init()` that calls `brickrust::init()` and registers hooks.

## Key macros

| Macro | Crate | Purpose |
|-------|-------|---------|
| `sig!("..." )` | brickrust_macros | Compile-time byte-pattern signature with `?`/`??` wildcards |
| `lookup! { ... }` | brickworks | Register function pointer lookups (binary, dll, proc, proc_mangled) |
| `bp_function!(...)` | brickrust | Register blueprint function overrides via inventory |
| `container_of!(ptr, Type, field)` | brickrust_macros | Calculate struct base pointer from field pointer |
| `set_module_name!(b"...")` | brickworks | Set module prefix for `br_print!()` output |
| `br_print!(...)` | brickworks | Debug logging (calls `OutputDebugStringA` + `brickworks_puts`) |
| `warn_version_mismatch!()` | brickrust | Warn if game version doesn't match mod's `game_version` |
| `panic_version_mismatch!()` | brickrust | Panic if game version doesn't match mod's `game_version` |

## BRMK (BrickRigs Mod Kit Steam)

When the `brmk` feature is enabled, brickworks builds as a plugin DLL (`BrickRigsModKitSteam-BrickRust.dll`) instead of a standalone DLL. The `brmk_plugin/` crate exports `InitializeModule()` which is called by the game's plugin system. BRMK uses a different lookup model: signatures scan plugin DLLs (`BrickRigsModKitSteam-*.dll`) instead of the main binary, and function pointers resolve via C++ decorated names.

## Utility functions

- `copy_vtable_estimate_size()` / `copy_vtable()` — copy and size a vtable to writable memory
- `uclass_reserve_memory2::<T>()` — expand `UClass` size to embed `T` inline
- `GObjects()` — iterate `FUObjectArray` via `FUObjectArrayIter`
- `GNames()` — access `FNamePool` for name table lookup
- `FName::search_str()` — find or allocate a `FName` from a string (with hashmap caching)
