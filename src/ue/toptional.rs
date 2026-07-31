#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TOptional<T>
{
    pub value: T,
    pub is_set: u8,
}
