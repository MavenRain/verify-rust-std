pub fn shared_identity<T>(value: &[T]) -> &[T] {
    value
}

pub fn shared_pair<T>(value: &[T]) -> (&[T], &[T]) {
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
