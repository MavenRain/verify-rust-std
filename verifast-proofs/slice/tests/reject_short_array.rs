/*@
lem reject_short_array<T, N>(t: thread_id_t, p: *T, n: usize)
    req p != 0 &*& p[..n] |-> ?values &*& foreach(values, (own)(t)) &*&
        0 <= n &*& n < usize_of_const(typeid(N));
    ens array_full_borrow_content::<T, N>(t, p as *[T; N])();
{
    array_to_Array::<T, N>(p as *[T; N]);
    Array_elems_Array_of_elems::<T, N>(values);
    close array_own::<T, N>(t, Array_of_elems::<T, N>(values));
    close array_full_borrow_content::<T, N>(t, p as *[T; N])();
}
@*/
