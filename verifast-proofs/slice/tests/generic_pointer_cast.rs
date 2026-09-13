/*@
lem generic_pointer_cast<T: ?Sized, U: ?Sized>(p: *T)
    req true;
    ens p as *U == ptr_cast::<T, U>(p);
{
}
@*/
