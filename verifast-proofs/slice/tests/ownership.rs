pub fn shared_identity<T>(value: &[T]) -> &[T] {
    value
}

pub fn shared_pair<'a, T>(value: &'a [T]) -> (&'a [T], &'a [T]) {
    //@ close slice_share::<T>('a, _t, value);
    //@ leak slice_share::<T>('a, _t, value);
    //@ close_ref_own::<'a, [T]>(value);
    //@ close_ref_own::<'a, [T]>(value);
    (value, value)
}

pub fn exclusive_identity<T>(value: &mut [T]) -> &mut [T] {
    value
}

pub fn shared_len<T>(value: &[T]) -> usize {
    value.len()
}

pub fn interior_mutable_identity<T>(value: &[std::cell::Cell<T>]) -> &[std::cell::Cell<T>] {
    value
}
