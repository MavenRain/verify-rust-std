/*@
lem generic_pointer_cast<T: ?Sized, U: ?Sized>(p: *T)
    req true;
    ens p as *U == ptr_cast::<T, U>(p);
{
}
@*/

pub fn const_pointer_cast<T: ?Sized, U>(pointer: *const T) -> *const U
//@ req true;
//@ ens result == pointer as *const U;
//@ on_unwind_ens false;
{
    pointer.cast()
}

pub fn mut_pointer_cast<T: ?Sized, U>(pointer: *mut T) -> *mut U
//@ req true;
//@ ens result == pointer as *mut U;
//@ on_unwind_ens false;
{
    pointer.cast()
}
