
pub const DEFAULT_ALIGNMENT: u32 = 0u32;

unsafe extern "C" {
    #[link_name = "BrickRust_FMalloc_Malloc"]
    pub fn malloc( count: usize, alignment: u32 ) -> *mut ();
    #[link_name = "BrickRust_FMalloc_Realloc"]
    pub fn realloc( original: *mut(), count: usize, alignment: u32 ) -> *mut ();
    #[link_name = "BrickRust_FMalloc_Free"]
    pub fn free( original: *mut() ) -> bool;
    #[link_name = "BrickRust_FMalloc_GetAllocationSize"]
    pub fn get_allocation_size( original: *mut(), size_out: *mut usize ) -> bool;
}
