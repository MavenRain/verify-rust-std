pub fn array_identity<T, const N: usize>(value: [T; N]) -> [T; N] {
    value
}

pub fn array_shared_identity<T, const N: usize>(value: &[T; N]) -> &[T; N] {
    value
}

pub fn array_exclusive_identity<T, const N: usize>(value: &mut [T; N]) -> &mut [T; N] {
    value
}

pub fn array_length<T, const N: usize>(_value: &[T; N]) -> usize {
    N
}
