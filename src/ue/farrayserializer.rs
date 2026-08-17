pub use super::tmap::TMap;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FFastArraySerializer
{
    item_map: TMap<i32, i32>,
    id_counter: i32,
    repl_key: i32,
    guid_ref_map: TMap<i32, ()>,
    guid_ref_map_struct_delta: TMap<i32, ()>,
}
