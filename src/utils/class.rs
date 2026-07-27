use crate::ue::fmalloc;

fn align( size: usize, align: usize ) -> usize {
    (size + align - 1) & !(align - 1)
}
pub unsafe fn class_extend<C,T>( cls: *mut C, size: usize ) -> (*mut C, *mut T)
{
    let s = align(size, 8);
    let s2 = align(size_of::<T>(), 8);
    let mem = fmalloc::realloc(cls as *mut (), s+s2, 8) as *mut C;
    return (mem, mem.add(s) as *mut T);
}
pub unsafe fn class_extend_estimate_size<C, T>( cls: *mut C ) -> (*mut C, *mut T)
{
    let mut cls_size: usize = 0;
    fmalloc::get_allocation_size(cls as *mut (), &mut cls_size);
    class_extend(cls, cls_size)
}
