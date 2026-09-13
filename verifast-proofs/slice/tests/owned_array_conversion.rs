#[path = "../lemmas.rs"]
mod lemmas;

/*@
lem owned_array_roundtrip<T, N>(t: thread_id_t, p: *T)
    req p != 0 &*& slice_full_borrow_content::<T>(t, p, usize_of_const(typeid(N)))();
    ens slice_full_borrow_content::<T>(t, p, usize_of_const(typeid(N)))();
{
    lemmas::slice_to_owned_array::<T, N>(t, p);
    lemmas::owned_array_to_slice::<T, N>(t, p as *[T; N]);
}

lem empty_array_roundtrip<T, N>(t: thread_id_t, p: *T)
    req slice_ref_valid(p, 0) == true &*& usize_of_const(typeid(N)) == 0;
    ens slice_full_borrow_content::<T>(t, p, 0)();
{
    close array::<T>(p, 0, nil);
    close foreach(nil, (own)(t));
    close slice_full_borrow_content::<T>(t, p, 0)();
    owned_array_roundtrip::<T, N>(t, p);
}
@*/
