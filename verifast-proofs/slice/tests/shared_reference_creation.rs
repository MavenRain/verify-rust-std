pub unsafe fn some_from_shared<'a, T>(pointer: *const T) -> Option<&'a T>
//@ req thread_token(?t) &*& [_](<T>.share)('a, t, pointer) &*& [?q]lifetime_token('a);
    //@ ens thread_token(t) &*& [q]lifetime_token('a) &*& std::option::Option_own::<&'a T>(t, result);
    //@ on_unwind_ens false;
{
    //@ let p = precreate_ref(pointer);
    //@ produce_type_interp::<T>();
    //@ init_ref_share::<T>('a, t, p);
    //@ leak type_interp::<T>();
    //@ open_frac_borrow('a, ref_initialized_::<T>(p), q / 2);
    //@ open [?f]ref_initialized_::<T>(p)();
    let result = Some(unsafe { &*pointer });
    //@ close [f]ref_initialized_::<T>(p)();
    //@ close_frac_borrow(f, ref_initialized_::<T>(p));
    //@ let value = p as &'a T;
    //@ close_ref_own::<'a, T>(value);
    //@ close std::option::Option_own::<&'a T>(t, std::option::Option::Some(value));
    result
}
