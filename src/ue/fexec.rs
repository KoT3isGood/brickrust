#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FExec
{
    pub vtable: *mut usize,
}
