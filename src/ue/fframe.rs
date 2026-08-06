use crate::ue::uclass::UFunction;

use super::coreuobject::UObject;

#[repr(C)]
pub struct FFrame
{
    _pad1: u64,
    _pad2: u64,
    pub node: *mut UFunction,
    pub object: *mut UObject,
    pub code: *mut u8,
    pub locals: *mut u8,
    _pad3: u64,
    _pad4: u64,
    _pad5: u64,
    _pad6: u64,
    _pad7: u64,
    _pad8: u64,
    pub out_params: *mut (),
}

impl FFrame
{
    pub unsafe fn get_input_params<T>(&self) -> *mut T
    {
        self.out_params as *mut T
    }
}
