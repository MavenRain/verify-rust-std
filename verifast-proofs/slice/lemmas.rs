/*@
lem split_owned_elements<T>(t: thread_id_t, p: *T, n: usize, mid: usize)
    req slice_full_borrow_content::<T>(t, p, n)() &*& 0 <= mid &*& mid <= n;
    ens slice_full_borrow_content::<T>(t, p, mid)() &*&
        slice_full_borrow_content::<T>(t, p + mid, n - mid)();
{
    open slice_full_borrow_content::<T>(t, p, n)();
    assert p[..n] |-> ?values;
    array_split(p, mid);
    foreach_split(values, (own)(t), mid);
    close slice_full_borrow_content::<T>(t, p, mid)();
    close slice_full_borrow_content::<T>(t, p + mid, n - mid)();
}

lem join_owned_elements<T>(t: thread_id_t, p: *T, n: usize, mid: usize)
    req slice_full_borrow_content::<T>(t, p, mid)() &*&
        slice_full_borrow_content::<T>(t, p + mid, n - mid)();
    ens slice_full_borrow_content::<T>(t, p, n)();
{
    open slice_full_borrow_content::<T>(t, p, mid)();
    open slice_full_borrow_content::<T>(t, p + mid, n - mid)();
    assert p[..mid] |-> ?prefix;
    assert (p + mid)[..n - mid] |-> ?suffix;
    array_join(p);
    foreach_append(prefix, suffix);
    close slice_full_borrow_content::<T>(t, p, n)();
}

lem slice_to_owned_array<T, N>(t: thread_id_t, p: *T)
    req p != 0 &*& slice_full_borrow_content::<T>(t, p, usize_of_const(typeid(N)))();
    ens array_full_borrow_content::<T, N>(t, p as *[T; N])();
{
    open slice_full_borrow_content::<T>(t, p, usize_of_const(typeid(N)))();
    assert p[..usize_of_const(typeid(N))] |-> ?values;
    Array_elems_Array_of_elems::<T, N>(values);
    array_to_Array::<T, N>(p as *[T; N]);
    close array_own::<T, N>(t, Array_of_elems::<T, N>(values));
    close array_full_borrow_content::<T, N>(t, p as *[T; N])();
}

lem owned_array_to_slice<T, N>(t: thread_id_t, p: *[T; N])
    req array_full_borrow_content::<T, N>(t, p)();
    ens slice_full_borrow_content::<T>(t, p as *T, usize_of_const(typeid(N)))();
{
    open array_full_borrow_content::<T, N>(t, p)();
    open array_own::<T, N>(t, ?array);
    Array_to_array::<T, N>(p);
    close slice_full_borrow_content::<T>(t, p as *T, usize_of_const(typeid(N)))();
}

lem restore_owned_prefix<T, N>(t: thread_id_t, p: *T, n: usize, prefix: *[T; N])
    req array_full_borrow_content::<T, N>(t, prefix)() &*&
        ref_mut_end_token(prefix, p as *[T; N]) &*&
        slice_full_borrow_content::<T>(t, p + usize_of_const(typeid(N)), n - usize_of_const(typeid(N)))();
    ens slice_full_borrow_content::<T>(t, p, n)();
{
    open array_full_borrow_content::<T, N>(t, prefix)();
    end_ref_mut::<[T; N]>(prefix);
    close array_full_borrow_content::<T, N>(t, p as *[T; N])();
    owned_array_to_slice::<T, N>(t, p as *[T; N]);
    join_owned_elements(t, p, n, usize_of_const(typeid(N)));
}

lem split_shared_elements<T>(k: lifetime_t, t: thread_id_t, p: *T, n: usize, mid: usize)
    req [_]slice_elements_share(k, t, p, n) &*& 0 <= mid &*& mid <= n;
    ens [_]slice_elements_share(k, t, p, mid) &*&
        [_]slice_elements_share(k, t, p + mid, n - mid);
{
    if mid == 0 {
        close slice_elements_share(k, t, p, 0);
        leak slice_elements_share(k, t, p, 0);
    } else {
        open [_]slice_elements_share(k, t, p, n);
        split_shared_elements(k, t, p + 1, n - 1, mid - 1);
        close slice_elements_share(k, t, p, mid);
        leak slice_elements_share(k, t, p, mid);
    }
}

lem slice_prefix_valid<T>(p: *T, n: usize, mid: usize)
    req slice_ref_valid(p, n) == true &*& 0 <= mid &*& mid <= n;
    ens slice_ref_valid(p, mid) == true;
{
    mul_mono_l(0, mid, std::mem::size_of::<T>());
    mul_mono_l(mid, n, std::mem::size_of::<T>());
}

lem slice_suffix_valid<T>(k: lifetime_t, t: thread_id_t, p: *T, n: usize, mid: usize)
    req [_]slice_elements_share(k, t, p, n) &*&
        slice_ref_valid(p, n) == true &*& 0 <= mid &*& mid <= n;
    ens [_]slice_elements_share(k, t, p, n) &*&
        slice_ref_valid(p + mid, n - mid) == true;
{
    if mid != 0 {
        std::alloc::is_valid_layout_size_of_align_of::<T>();
        is_power_of_2_pos(std::mem::align_of::<T>());
        slice_prefix_valid(p, n, 1);
        mul_mono_l(0, n - 1, std::mem::size_of::<T>());
        mul_mono_l(n - 1, n, std::mem::size_of::<T>());
        size_align::<T>();
        div_rem(p as usize, std::mem::align_of::<T>());
        div_rem(std::mem::size_of::<T>(), std::mem::align_of::<T>());
        div_rem_nonneg_unique((p + 1) as usize, std::mem::align_of::<T>(),
            p as usize / std::mem::align_of::<T>() +
            std::mem::size_of::<T>() / std::mem::align_of::<T>(), 0);
        open slice_elements_share(k, t, p, n);
        slice_suffix_valid(k, t, p + 1, n - 1, mid - 1);
        close slice_elements_share(k, t, p, n);
        leak slice_elements_share(k, t, p, n);
    }
}
@*/
