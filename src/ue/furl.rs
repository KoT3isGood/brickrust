#![allow(nonstandard_style)]

use crate::ue::fname::FName;
use crate::ue::fstring::FString;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct FURL {
    pub object_name: FName,
    pub site: FString,
    pub options: FString,
    pub travel_from: FString,
    pub b_seamless: bool,
    pub b_authority: bool,
    pub b_elo : bool,
    pub pad: u64,
    pub unique_id: u64,
    pub pad2: u64,
}
