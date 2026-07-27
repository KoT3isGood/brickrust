
use crate::br::properties::reflection::FBrickPropertyReflection;

#[repr(C)]
#[derive(Debug)]
pub struct IBrickPropertyInterfaceVTable
{
    pub _a_0: unsafe extern "C" fn( brick: *const IBrickPropertyInterface ),
    pub getUObject: unsafe extern "C" fn( brick: *const IBrickPropertyInterface ),
    pub ReflectBrickProperties: unsafe extern "C" fn( brick: *const IBrickPropertyInterface, reflection: *mut FBrickPropertyReflection ),
    pub CanModifyBrickProperty: unsafe extern "C" fn( brick: *const IBrickPropertyInterface ),
    pub PostModifyBrickProperty: unsafe extern "C" fn( brick: *const IBrickPropertyInterface ),
    pub UpdateFocusedBrickProperty: unsafe extern "C" fn( brick: *const IBrickPropertyInterface ),
}

#[repr(C)]
pub struct IBrickPropertyInterface
{
    pub vtable: *mut usize,
}
