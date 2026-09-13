pub unsafe fn missing_shared_elements<'a, T>(pointer: *const T) -> Option<&'a T>
//@ req thread_token(?t) &*& [?q]lifetime_token('a);
    //@ ens true;
{
    //@ let p = precreate_ref(pointer);
    //@ produce_type_interp::<T>();
    //@ init_ref_share::<T>('a, t, p);
    Some(unsafe { &*pointer })
}
