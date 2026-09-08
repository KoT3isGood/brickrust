//!
//! This crate is a basic mod loader for Brick Rigs, made to load dynamic libraries.
//! Provides basic logging and signature finding for mods.
//!
//! # Folder structure
//! This is required folder structure for windows
//! ```txt
//! BrickRigs.exe
//! ...
//! BrickRigs/
//!     Mods/
//!         YourMod/
//!             Content/
//!                 ...
//!             Yourmod.uplugin
//!             mod.dll
//!     Binaries/
//!         Win64/
//!              BrickRigsSteam-Win64-Shipping.exe
//!              brickworks.dll
//!              xinput1_3.dll
//! brickworks/
//!     yourmod.dll      # enabled mod
//!     _yourmod2.dll    # disabled mod
//! brickworks.txt
//! libgcc_s_seh-1.dll      # shared dependencies
//! libwinpthread-1.dll
//! ```
//!
//! # Mod prerequisites
//!
//! We have few requirements for a mod to get loaded. All the functions below must follow C ABI.
//!
//! ## Loading paths
//! - `brickworks/*.dll`
//! - `BrickRigs/Mods/*/mod.dll`
//! 
//! ## Mandatory functions
//! ```
//! #[unsafe(no_mangle)]
//! unsafe extern "C" fn mod_info() -> ModInfo
//! {
//!     ...
//! }
//!
//! #[unsafe(no_mangle)]
//! unsafe extern "C" fn mod_init()
//! {
//!     ...
//! }
//! ```
//! 
//!
//! ## Optional functions
//! ```
//! #[unsafe(no_mangle)]
//! unsafe extern "C" fn mod_deinit()
//! {
//!     ...
//! }
//! ```
//!


#![allow(static_mut_refs)]
pub mod print;
pub mod modinfo;
pub mod patterns;
pub mod iface;
