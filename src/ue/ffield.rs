use super::coreuobject::{EObjectFlags, UObjectBase};
use super::fname::FName;
use core::fmt;

#[repr(C)]
union FFieldObjectUnion
{
    field: *mut FField,
    object: *mut UObjectBase,
}

#[repr(C)]
pub struct FFieldVariant
{
    container: FFieldObjectUnion,
    b_is_uobject: bool,
}

impl fmt::Debug for FFieldVariant
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe
        {
            f.debug_struct("FFieldVariant")
                .field("container", &self.container.object)
                .field("b_is_uobject", &self.b_is_uobject)
                .finish()
        }
    }

}

#[derive(Debug)]
#[repr(C)]
pub struct FFieldClass {
    // TODO
    pub name: FName,
}

#[derive(Debug)]
#[repr(C)]
pub struct FField {
    pub class_private: *const FFieldClass,
    pub owner: FFieldVariant,
    pub next: *const FField,
    pub name_private: FName,
    pub flags_private: EObjectFlags,
}
