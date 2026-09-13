#![feature(rustc_attrs)]
#![rustc_coherence_is_core]
#![allow(internal_features)]

impl<T> [T] {
    pub const fn first_chunk<const N: usize>(&self) -> Option<&[T; N]> {
        if self.len() < N {
            None
        } else {
            // SAFETY: We explicitly check for the correct number of elements,
            //   and do not let the reference outlive the slice.
            Some(unsafe { &*(self.as_ptr().cast_array()) })
        }
    }
}

impl<T> *const T {
    pub const fn cast_array<const N: usize>(self) -> *const [T; N] {
        self.cast()
    }
}
