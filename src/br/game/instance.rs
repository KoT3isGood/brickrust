use crate::ue::tarray::*;

#[allow(non_upper_case_globals)]
pub static mut GetEnabledModNames: 
Option<unsafe extern "C" fn ( out_names: *mut TArray<FString> )> = None;
