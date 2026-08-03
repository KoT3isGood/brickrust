
use crate::ue::fmalloc;
use core::ptr;
///
/// estimates the size of vtable by finding null function, may not give actual size
/// kinda bad
///
pub unsafe fn vtable_estimate_size( vtable: *mut usize ) -> usize
{
    let mut count = 0;
    while *vtable.add(count) != 0
    {
        count+=1;
    }

    count
}

pub unsafe fn allocate_vtable( count: usize ) -> *mut usize
{
    fmalloc::malloc(count * size_of::<usize>() ) as *mut usize
}


///
/// creates vtable copy
///
pub unsafe fn copy_vtable( vtable: *mut usize, count: usize ) -> *mut usize
{
    let new: *mut usize = allocate_vtable(count);
    ptr::copy(vtable, new, count);
    return new;
}

pub unsafe fn copy_vtable2<T>( vtable: *const T ) -> *mut T
{
    copy_vtable(vtable as *mut usize, size_of::<T>()/size_of::<usize>()) as *mut T
}

pub unsafe fn copy_vtable_with_parent( vtable: *mut usize, count: usize ) -> *mut usize
{
    let new: *mut usize = allocate_vtable(count+1);
    ptr::copy_nonoverlapping(vtable, new.add(1), count);
    *(new as *mut *mut usize) = vtable;
    return new.add(1);
}


///
/// creates vtable copy by using estimated size
/// returns pointer to new table and the size
pub unsafe fn copy_vtable_estimate_size( vtable: *mut usize ) -> (*mut usize, usize)
{
    let count = vtable_estimate_size(vtable);
    return (copy_vtable(vtable, count), count);
}


///
/// creates vtable copy by using estimated size
/// returns pointer to new table and the size
///
pub unsafe fn copy_vtable_estimate_size_with_parent( vtable: *mut usize ) -> (*mut usize, usize)
{
    let count = vtable_estimate_size(vtable);
    return (copy_vtable_with_parent(vtable, count), count);
}

///
/// creates new vtable and gives indexes to the subtable starts
/// user must fill the subtables after creation
/// returns pointer to new vtable, the size and indexes of subtables
///
pub unsafe fn vtable_add_custom_tables
    <const N: usize>( vtable: *mut usize, count: usize, customs_sizes: [usize; N] ) 
    -> ( *mut usize, usize, [usize; N])
{
    let mut vcount = count;
    for c in customs_sizes
    {
        vcount+=c;
    }

    let new: *mut usize = fmalloc::malloc
        (vcount * size_of::<usize>()) as *mut usize;
    ptr::copy_nonoverlapping(vtable, new, count);

    let mut index = count;
    let mut indexes = [0usize; N];
    for i in 0..N
    {
        indexes[i] = index;
        index += customs_sizes[i];
    }
    return (new, vcount, indexes)
}
pub unsafe fn class_vtable_clone<T>( vtable: *mut *mut usize)
{
    *vtable = copy_vtable(*vtable, size_of::<T>()/size_of::<usize>() );
}

pub unsafe fn class_vtable_clone_estimate_size( vtable: *mut *mut usize )
{
    *vtable = copy_vtable_estimate_size(*vtable).0;
}

pub unsafe fn class_vtable_clone_estimate_size_with_parent( vtable: *mut *mut usize )
{
    *vtable = copy_vtable_estimate_size_with_parent(*vtable).0;
}

pub unsafe fn class_get_parent_vtable( class: *mut *mut *mut () ) -> *mut usize
{
    *(*class).sub(1) as *mut usize
}


