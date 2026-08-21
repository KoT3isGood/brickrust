# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

BrickRust is an SDK and mod loader for the game **Brick Rigs** (Steam ID: 552100), written in unsafe Rust. It provides tooling to create mods that hook into the Unreal Engine 4 runtime used by the game, intercept function calls, and extend game functionality via brick property customization.

The project is a cargo workspace with four crates:
- **brickrust** — the main SDK crate (target: `x86_64-pc-windows-gnu`, dylib). Maps UE4 structures, hooks functions, provides mod APIs.
- **brickworks** — the mod loader DLL. Scans a `brickworks/` folder for `.dll` mod files, initializes them via `mod_info`/`mod_init`, provides signature lookup and hooking infrastructure.
- **brickrust_macros** — proc-macro crate providing the `sig!` macro for compile-time byte-pattern signatures.
- **xinput_proxy** — a shim DLL that forwards xinput1_3.dll calls and loads brickworks.dll at startup.

## Building and Installing

```sh
# Default target is x86_64-pc-windows-gnu (set in .cargo/config.toml)

# Release build + install to Brick Rigs folder
make install DIR="/path/to/common/Brick Rigs"

# Development build
make install DIR="/path/to/common/Brick Rigs" dev=true

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

## Architecture

### Entry points

Mods must export two symbols (no mangling):
- `mod_info()` → returns a `ModInfo` struct with name, description, version, game_version, authors
- `mod_init()` → called before engine init; must call `brickrust::init()` and set up hooks

The mod loader (brickworks) scans `brickworks/*.dll`, loads each, resolves `mod_info` and `mod_init`, and runs them.

### Hooking system

The hooking pipeline uses **min_hook_rs** (EtwHook) for inline function hooking. Key hooks:

1. **UEngine::Init** — `hook_post_engine_init()` registers callbacks fired after engine init
2. **UEngine::LoadMap** — `hook_post_load_map()` fires when a map loads
3. **StaticConstructObject_Internal** — `hook_construct_uobject()` intercepts every UE4 object construction, dispatching to subhooks

Subhook callbacks receive either `*mut UObjectBase` (object init) or raw function pointers (post-init/map callbacks).

### Signature scanning

Functions are located by byte-pattern signatures at runtime via `brickworks::patterns::lookup()`. The `sig!()` macro (from brickrust_macros) compiles byte patterns with `?`/`??` wildcards into `Signature` structs.

### UE4 structure mapping (`src/ue/`)

Maps core Unreal Engine 4 types:
- `coreuobject.rs` — `UObject`, `UObjectBase`, `FUObjectArray`, vtables, object iteration
- `uclass.rs` — `UClass`, `UStruct`
- `fname.rs` — `FName` (UE4 name system)
- `fproperty.rs` — `FProperty` and vtables
- `fstring.rs` / `ftext.rs` — string/UScriptName types
- `fframe.rs` — `FFrame` (UE4 execution stack frames)
- `tarray.rs` / `tmap.rs` / `tpair.rs` / `tshared.rs` / `toptional.rs` — UE4 containers
- `blueprint.rs` — Blueprint function interception via `inventory`-driven dispatch
- `gameplay/` — `AActor`, `UWorld`, `FActorSpawnParameters`, `GWorld`, `SpawnActor`

### External header reference

BrickRigs uses a fork of Unreal Engine 4. For UE4 types and signatures that don't map directly to this project's headers, use:

- **[Redacted00/BrickRigs-Headers](https://github.com/Redacted00/BrickRigs-Headers)** — community-maintained Brick Rigs header definitions

Note: these headers are outdated and may not match the exact game version (1.11.x), but they provide useful reference for UE4 struct layouts and function signatures used throughout the codebase.

### Brick Rigs game structures (`src/br/`)

Game-specific types:
- `bricks/` — `UBrick`, `UScalableBrick`, `UMotor`, `UActuator`
- `vehicle/` — `UBrickVehicle`, `UBrickConnection`, `InputChannel`, `InputAxis`, `Repmovement`
- `game/` — `GameInstance`, `SpawnPoint`, game lifecycle
- `properties/` — `IBrickPropertyInterface`, `FProperty`, `FNumericBrickPropertyBase`, `FBrickPropertyEditInfo`, property reflection system
- `items/` — inventory system
- `projectiles/` — ammo types
- `brickeditor/` — save/load for brick layouts
- `statics.rs` — static game function signatures

### VTable and memory manipulation (`src/utils/`, `src/really_scary.rs`)

- `utils/vtable.rs` — copies vtables to writable memory so mods can override methods
- `really_scary.rs` — dangerously expands `UClass` allocation size to embed custom struct fields inline

### Mod dispatch

The `inventory` crate stores `BlueprintFunction` structs; during `ProcessEvent` interception, the engine dispatches to registered blueprint functions by matching function name and class.

### Field naming convention (`src/ue/` only)

Two distinct naming styles are used depending on the struct kind:

- **VTable structs** (e.g., `UObjectVTable`, `AActorVTable`) — field names use **PascalCase**, matching UE4's C++ vtable method names (e.g., `Destructor`, `PostLoad`, `ProcessEvent`)
- **Data structs** (e.g., `UObjectBase`, `AActor`, `FActorSpawnParameters`) — field names use **snake_case** (e.g., `vtable`, `object_flags`, `internal_index`)

### Field naming convention (`src/br/` only)

All variable names are taken directly from the [Redacted00/BrickRigs-Headers](https://github.com/Redacted00/BrickRigs-Headers) repository. Data struct field names match the header definitions exactly.

For fields with unknown purposes (due to incomplete header definitions, reverse engineering gaps, or game version mismatches), the convention `_aX` is used where `X` is an incrementing index starting from 1 (e.g., `_a1`, `_a2`, `_a3`). This placeholder naming matches the pattern used when the exact purpose or type of a field cannot be determined.

Bitflag variants use **SCREAMING_SNAKE_CASE** without the `b` prefix (e.g., `IS_RUNNING`, `WAS_ON_THROTTLE`, `MANUALLY_SHIFTED`), omitting the `b` prefix that appears on the C++ header boolean fields (e.g., `bIsRunning`, `bWasOnThrottle`).

To create new bitflags: the struct type uses the `{TypeName}Flags` naming (e.g., `UMotorBrickFlags`), with the data struct field named `flags`. When a struct has multiple bitflag fields, each gets a numeric suffix — struct `UMotorBrickFlagsX`, variable `flagsX`.

Bitflag struct sizes must match the C++ layout: use `u8` for packed `uint8` fields, `u16` for `uint16`, `u32` for `uint32`, `u64` for `uint64`.

## DLL / Engine / Map lifecycle

The mod loader and SDK follow a strict initialization pipeline. Understanding when each stage fires is critical for correct API usage.

### Stage 1: DLL load (brickworks.dll)

When the game loads `brickworks.dll`, `brickworks_init()` runs. This function:
1. Initializes the hook manager (`min_hook_rs`)
2. Scans the `brickworks/` directory for `.dll` files (prefixed `_` = disabled)
3. Loads each mod DLL via `libloading`
4. Resolves `mod_info` from each mod and prints metadata
5. Calls each mod's `mod_init()`

At this point **no engine functions are available**. This stage is for setting up vtable hooks and early signatures.

### Stage 2: Engine init (`UEngine::Init`)

After the game's own `UEngine::Init` completes, the SDK fires all callbacks registered via `hook_post_engine_init()`. This is when:
- `init_signatures()` has resolved all UE4 and game function pointers
- Blueprint functions are ready for dispatch
- You can call engine functions like `warn_version_mismatch!()` and `check_blueprint_mod()`

In a mod's `mod_init()`, register callbacks with:
```rust
brickrust::hook_post_engine_init(my_engine_init);
```

The SDK's own `engine_load()` callback (registered internally) scans `GObjects()` and sets `UCLASS` to the "Class" UClass.

### Stage 3: Map load (`UEngine::LoadMap`)

When a level/map loads, `UEngine::LoadMap` is hooked. After the game's map load completes, all callbacks registered via `hook_post_load_map()` fire. At this point:
- `GObjects()` contains the full object graph for the current level
- `GNames()` has the full name table
- Actor components are constructible and traversable

Register with:
```rust
brickrust::hook_post_load_map(my_map_init);
```

### Stage 4: Object construction (ongoing)

`StaticConstructObject_Internal` is hooked for the lifetime of the process. Every time UE4 creates an object (via `StaticConstructObject_Internal`), all subhooks registered via `hook_construct_uobject()` receive the new `*mut UObjectBase`. This is the primary mechanism for:
- Replacing vtables on specific class types
- Injecting custom data into new instances
- Intercepting CDO (Class Default Object) creation

In a mod:
```rust
brickrust::hook_construct_uobject(my_object_init);
```

### Summary table

| Stage | Trigger | APIs available | Registration |
|-------|---------|----------------|--------------|
| DLL load | Game loads brickworks.dll | None (pre-engine) | — |
| Engine init | After `UEngine::Init` | Version checks, blueprint checks | `hook_post_engine_init()` |
| Map load | After `UEngine::LoadMap` | Full object traversal, GObjects, GNames | `hook_post_load_map()` |
| Object construction | Every `StaticConstructObject_Internal` call | Vtable manipulation, instance data | `hook_construct_uobject()` |

## Key development patterns

- All engine/game function pointers are **resolved at runtime**, not linked at compile time
- All code interacting with the game uses `unsafe` — raw pointers, vtable manipulation, transmute
- Structs are `#[repr(C)]` and manually aligned to match the game's memory layout
- Use `container_of!` macro from brickrust_macros for pointer arithmetic
- Module names are set via `set_module_name!()` for log routing

## Important constraints

- **Target only**: `x86_64-pc-windows-gnu`. The Cargo.toml and .cargo/config.toml both pin this.
- **Panics on abort**: Both dev and release profiles use `panic = "abort"` — no unwinding.
- **No native Linux support**: The game's Linux modding is incomplete; focus on Windows target.
- **Runtime dependencies**: The game binary needs `xinput1_3.dll`, `brickworks.dll`, and mingw runtime DLLs in the right paths.
- **Version checks**: Mods should run `warn_version_mismatch!()` or `panic_version_mismatch!()` after engine init (not in `mod_init`).
- **Blueprint mod checks**: Use `check_blueprint_mod()` / `ensure_blueprint_mod()` to validate required mods.

## File conventions

- Module files in `src/` use lowercase with `mod.rs` for module roots
- `pub(crate)` for internal-only APIs, `pub` for mod-facing APIs
- Engine/game function pointers stored as `Option<unsafe extern "C" fn(...)>`
- Vtable pointers cast via `transmute` to typed function pointers
