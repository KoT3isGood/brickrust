use brickworks::br_print;

use crate::ue::{fproperty::FProperty, uclass::UFunction};

use super::coreuobject::UObject;
use crate::BrickRust_print;

#[repr(C)]
pub struct FOutParmRec
{
    pub prop: *mut FProperty,
    pub addr: *mut (),
    pub next: *mut FOutParmRec,
}

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
    pub out_params: *mut FOutParmRec,
}

impl FFrame
{
    pub unsafe fn get_input_params<T>(&self) -> *mut T
    {
        self.locals as *mut T
    }
    pub unsafe fn set_output<T: Copy>(&self, name: &'static str, val: T) -> bool
    {
        let mut param = self.out_params;
        while !param.is_null() {
            let prop = (*param).prop;
            if (*prop).field.name_private.equals_str(name) {
                std::ptr::write((*param).addr as *mut T, val);
                return true;
            }
            param = (*param).next;
        }
        false
    }
}
