use bitflags::bitflags;


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FStructuredArchiveFormatter
{
    pub vtable: *mut usize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FStructuredArchiveSlot
{
    pub depth: i32,
    pub element_id: u32,
    pub archive: *mut FStructuredArchive
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FStructuredArchive
{
    pub formatter: *mut FStructuredArchiveFormatter
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FStructuredArchiveFromArchive
{
    impl_storage: [u64; 50],
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct FArchiveStateFlags1: u8 {
        const ArAllowLazyLoading  = 0b0001;
        const ArContainsCode = 0b0010;
        const ArContainsMap  = 0b0100;
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct FEngineVersionBase
{
    major: u16,
    minor: u16,
    patch: u16,
    changelist: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(nonstandard_style)]
pub struct FArchiveState
{
    pub vtable: *mut usize,
    pub flags1: FArchiveStateFlags1,
    pub ArCustomPropertyList: *const (),
    pub ArDebugSerializationFlags: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FArchive
{
}
