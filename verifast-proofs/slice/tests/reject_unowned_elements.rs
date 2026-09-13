/*@
lem unowned_elements<T>(t: thread_id_t, p: *T)
    req slice_ref_valid(p, 1) == true &*& p[..1] |-> ?values;
    ens slice_full_borrow_content::<T>(t, p, 1)();
{
    close slice_full_borrow_content::<T>(t, p, 1)();
}
@*/
