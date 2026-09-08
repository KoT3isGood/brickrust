unsafe extern "C"
{
    fn fopen( path: *const u8, mode: *const u8 ) -> *mut ();
    fn fclose( stream: *mut () ) -> i32;
    fn fprintf( stream: *mut (), format: *const u8, ... ) -> i32;
    fn fflush( stream: *mut () ) -> i32;
    fn _lock_file( stream: *mut () );
    fn _unlock_file( stream: *mut () );
}


pub(crate) unsafe fn get_logger() -> *mut ()
{
    #[allow(non_upper_case_globals)]
    static mut brickworks_logger: *mut () = core::ptr::null_mut();
    if brickworks_logger.is_null()
    {
        brickworks_logger = fopen(b"brickworks.txt\0".as_ptr(), b"wb\0".as_ptr());
    }
    brickworks_logger
}

///
/// Puts a message into a logger.
/// During runtime it is written into brickworks.txt
/// In BRMK it is directly written into logs
///
#[no_mangle]
pub unsafe extern "C" fn brickworks_puts( modname: *const u8, value: *const u8 )
{
    let logger = get_logger();
    _lock_file(logger);
    fprintf(logger,b"[%s] %s\n\0".as_ptr(), modname, value);
    fflush(logger);
    _unlock_file(logger);
}
