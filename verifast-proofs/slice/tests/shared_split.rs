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
@*/
