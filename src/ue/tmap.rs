#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TMap<K, T> {
    pub num_pairs: i32,
    pub key_arena: *mut K,
    pub pair_arena: *mut T,
    pub num_buckets: i32,
    pub bucket_size: i32,
    pub b_has_pair_keys: u8,
    pub b_has_pair_values: u8,
    pub b_key_funcs_comparable: u8,
    pub _pad: u8,
    pub pair_info: *mut FPairInfo<K, T>,
    pub reserved_num_buckets: i32,
    pub _pad1: u8,
    pub _pad2: u8,
    pub _pad3: u8,
    pub pair_info_free_list: *mut FPairInfo<K, T>,
    pub bucket_array: *mut FDefaultBucket<K, T>,
    pub first_bucket: *mut FDefaultBucket<K, T>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FDefaultBucket<K, T> {
    pub hash_key: K,
    pub pair_info_ptr: *mut FPairInfo<K, T>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FPairInfo<K, T> {
    pub key: K,
    pub value: T,
    pub next: *mut FDefaultBucket<K, T>,
}
