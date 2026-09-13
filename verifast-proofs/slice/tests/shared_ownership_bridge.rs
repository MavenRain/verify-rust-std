/*@
lem share_twice<'a, T>(p: &'a [T], t: thread_id_t)
    req thread_token(t) &*& slice_ref_valid(p as *T, ptr_len(p)) == true &*&
        [_]slice_elements_share('a, t, p as *T, ptr_len(p));
    ens thread_token(t) &*& <&'a [T]>.own(t, p) &*& <&'a [T]>.own(t, p);
{
    close slice_share::<T>('a, t, p);
    leak slice_share::<T>('a, t, p);
    close_ref_own::<'a, [T]>(p);
    close_ref_own::<'a, [T]>(p);
}
@*/
