use super::patterns::*;

unsafe extern "C"
{
    pub fn brickworks_binary_lookup( _offset: isize, _mode: LookupMode, _sign: CSignature ) -> *const u8;
    pub fn brickworks_binary_dll_lookup( _dll: *const u8, _offset: isize, _mode: LookupMode, sign: CSignature ) -> *const u8;
    pub fn brickworks_cpp_lookup( _cpp: *const u8 ) -> *const u8;

    pub fn brickworks_get_base_address() -> *const ();
    pub fn brickworks_get_base_size() -> usize;

    pub fn brickworks_puts( _modname: *const u8, _value: *const u8 );

    pub fn brickworks_create_hook( f: *const (), new_fn: *const()) -> *const ();
    pub fn brickworks_add_prehook( f: *const (), sub: *const () );
    pub fn brickworks_get_prehooks( f: *const () ) -> *const *const ();
    pub fn brickworks_get_prehook_count( f: *const () ) -> usize;
    pub fn brickworks_add_posthook( f: *const (), sub: *const () );
    pub fn brickworks_get_posthooks( f: *const () ) -> *const *const ();
    pub fn brickworks_get_posthook_count( f: *const () ) -> usize;
}
