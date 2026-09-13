/*@
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
