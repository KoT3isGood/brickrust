
unsafe extern "C"
{
    fn fopen( path: *const u8, mode: *const u8 ) -> *mut ();
    fn fclose( stream: *mut () ) -> i32;
    fn fprintf( stream: *mut (), format: *const u8, ... ) -> i32;
    fn fflush( stream: *mut () ) -> i32;
    fn _lock_file( stream: *mut () );
    fn _unlock_file( stream: *mut () );
}

static mut LOGGER: *mut () = core::ptr::null_mut();

pub(crate) unsafe fn init()
{
    LOGGER = fopen(b"brickworks.txt\0".as_ptr(), b"wb\0".as_ptr());
}

pub(crate) unsafe fn deinit()
{
    fclose(LOGGER);
}

/**
 * Puts a message into a logger.
 * */
#[no_mangle]
pub unsafe fn brickworks_puts( modname: *const u8, value: *const u8 )
{
    _lock_file(LOGGER);
    fprintf(LOGGER,b"[%s] %s\n\0".as_ptr(), modname, value);
    fflush(LOGGER);
    _unlock_file(LOGGER);
}
