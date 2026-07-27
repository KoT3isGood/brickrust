use super::modinfo::ModInfo;

#[macro_export]
macro_rules! container_of {
    ($ptr:expr, $type:ty, $field:ident) => {{
        use core::mem::offset_of;
        let field_ptr = $ptr as *const u8;
        let offset = offset_of!($type, $field);

        (field_ptr.sub(offset) as *const $type)
    }};
}
