pub mod universal;
#[cfg(all(feature="impl", feature = "brmk"))]
pub mod brmk;
#[cfg(all(feature="impl", target_os = "windows", not(feature = "brmk")))]
pub mod win32;

#[cfg(all(feature="impl", target_os = "windows"))]
pub mod win_universal;

#[cfg(not(feature = "impl"))]
pub mod stub;

pub mod hookmgr;
pub use brickworks::*;
