/*@
lem fabricated_slice<T>(t: thread_id_t, p: *T, n: usize)
    req slice_ref_valid(p, n) == true &*& 0 < n;
    ens slice_full_borrow_content::<T>(t, p, n)();
{
    close slice_full_borrow_content::<T>(t, p, n)();
}
@*/
