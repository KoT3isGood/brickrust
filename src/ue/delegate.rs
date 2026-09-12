#![allow(nonstandard_style)]
use crate::ue::tarray::TArray;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TMulticastDelegate
{
    pub InvocationList: TArray<()>,
    pub CompactionThreshold: i32,
    pub InvocationListLockCount: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TDelegate
{
    pub _a01: usize,
    pub _a02: usize,
    pub _a03: usize,
    pub _a04: usize,
}

